# AST Trace: ../rust/compiler/rustc_type_ir/src/fold.rs

Generated 28 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=27 | LINES=52

```rust
// A folding traversal mechanism for complex data structures that contain type
// information.
//
// This is a modifying traversal. It consumes the data structure, producing a
// (possibly) modified version of it. Both fallible and infallible versions are
// available. The name is potentially confusing, because this traversal is more
// like `Iterator::map` than `Iterator::fold`.
//
// This traversal has limited flexibility. Only a small number of "types of
// interest" within the complex data structures can receive custom
// modification. These are the ones containing the most important type-related
// information, such as `Ty`, `Predicate`, `Region`, and `Const`.
//
// There are three traits involved in each traversal.
// - `TypeFoldable`. This is implemented once for many types, including:
//   - Types of interest, for which the methods delegate to the folder.
//   - All other types, including generic containers like `Vec` and `Option`.
//     It defines a "skeleton" of how they should be folded.
// - `TypeSuperFoldable`. This is implemented only for recursive types of
//   interest, and defines the folding "skeleton" for these types. (This
//   excludes `Region` because it is non-recursive, i.e. it never contains
//   other types of interest.)
// - `TypeFolder`/`FallibleTypeFolder`. One of these is implemented for each
//   folder. This defines how types of interest are folded.
//
// This means each fold is a mixture of (a) generic folding operations, and (b)
// custom fold operations that are specific to the folder.
// - The `TypeFoldable` impls handle most of the traversal, and call into
//   `TypeFolder`/`FallibleTypeFolder` when they encounter a type of interest.
// - A `TypeFolder`/`FallibleTypeFolder` may call into another `TypeFoldable`
//   impl, because some of the types of interest are recursive and can contain
//   other types of interest.
// - A `TypeFolder`/`FallibleTypeFolder` may also call into a `TypeSuperFoldable`
//   impl, because each folder might provide custom handling only for some types
//   of interest, or only for some variants of each type of interest, and then
//   use default traversal for the remaining cases.
//
// For example, if you have `struct S(Ty, U)` where `S: TypeFoldable` and `U:
// TypeFoldable`, and an instance `s = S(ty, u)`, it would be folded like so:
// ```text
// s.fold_with(folder) calls
// - ty.fold_with(folder) calls
//   - folder.fold_ty(ty) may call
//     - ty.super_fold_with(folder)
// - u.fold_with(folder)
// ```

use std::convert::Infallible;
use std::mem;
use std::sync::Arc;

use crate::rustc_index::{Idx, IndexVec};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use thin_vec::ThinVec;
use tracing::{debug, instrument};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use crate::inherent::*;
use crate::visit::{TypeVisitable, TypeVisitableExt as _};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::{self as ty, Interner, TypeFlags};
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=try_fold_with | COMPLEXITY=17 | LINES=40

```rust
/// This trait is implemented for every type that can be folded,
/// providing the skeleton of the traversal.
///
/// To implement this conveniently, use the derive macro located in
/// `rustc_macros`.
///
/// This trait is a sub-trait of `TypeVisitable`. This is because many
/// `TypeFolder` instances use the methods in `TypeVisitableExt` while folding,
/// which means in practice almost every foldable type needs to also be
/// visitable. (However, there are some types that are visitable without being
/// foldable.)
pub trait TypeFoldable<I: Interner>: TypeVisitable<I> + Clone {
    /// The entry point for folding. To fold a value `t` with a folder `f`
    /// call: `t.try_fold_with(f)`.
    ///
    /// For most types, this just traverses the value, calling `try_fold_with`
    /// on each field/element.
    ///
    /// For types of interest (such as `Ty`), the implementation of this method
    /// calls a folder method specifically for that type (such as
    /// `F::try_fold_ty`). This is where control transfers from [`TypeFoldable`]
    /// to [`FallibleTypeFolder`].
    fn try_fold_with<F: FallibleTypeFolder<I>>(self, folder: &mut F) -> Result<Self, F::Error>;

    /// The entry point for folding. To fold a value `t` with a folder `f`
    /// call: `t.fold_with(f)`.
    ///
    /// For most types, this just traverses the value, calling `fold_with`
    /// on each field/element.
    ///
    /// For types of interest (such as `Ty`), the implementation of this method
    /// calls a folder method specifically for that type (such as
    /// `F::fold_ty`). This is where control transfers from `TypeFoldable`
    /// to `TypeFolder`.
    ///
    /// Same as [`TypeFoldable::try_fold_with`], but not fallible. Make sure to keep
    /// the behavior in sync across functions.
    fn fold_with<F: TypeFolder<I>>(self, folder: &mut F) -> Self;
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=try_super_fold_with | COMPLEXITY=11 | LINES=19

```rust
// This trait is implemented for types of interest.
pub trait TypeSuperFoldable<I: Interner>: TypeFoldable<I> {
    /// Provides a default fold for a recursive type of interest. This should
    /// only be called within `TypeFolder` methods, when a non-custom traversal
    /// is desired for the value of the type of interest passed to that method.
    /// For example, in `MyFolder::try_fold_ty(ty)`, it is valid to call
    /// `ty.try_super_fold_with(self)`, but any other folding should be done
    /// with `xyz.try_fold_with(self)`.
    fn try_super_fold_with<F: FallibleTypeFolder<I>>(
        self,
        folder: &mut F,
    ) -> Result<Self, F::Error>;

    /// A convenient alternative to `try_super_fold_with` for use with
    /// infallible folders. Do not override this method, to ensure coherence
    /// with `try_super_fold_with`.
    fn super_fold_with<F: TypeFolder<I>>(self, folder: &mut F) -> Self;
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=cx | COMPLEXITY=16 | LINES=42

```rust
/// This trait is implemented for every infallible folding traversal. There is
/// a fold method defined for every type of interest. Each such method has a
/// default that does an "identity" fold. Implementations of these methods
/// often fall back to a `super_fold_with` method if the primary argument
/// doesn't satisfy a particular condition.
///
/// A blanket implementation of [`FallibleTypeFolder`] will defer to
/// the infallible methods of this trait to ensure that the two APIs
/// are coherent.
pub trait TypeFolder<I: Interner>: Sized {
    fn cx(&self) -> I;

    fn fold_binder<T>(&mut self, t: ty::Binder<I, T>) -> ty::Binder<I, T>
    where
        T: TypeFoldable<I>,
    {
        t.super_fold_with(self)
    }

    fn fold_ty(&mut self, t: I::Ty) -> I::Ty {
        t.super_fold_with(self)
    }

    // The default region folder is a no-op because `Region` is non-recursive
    // and has no `super_fold_with` method to call.
    fn fold_region(&mut self, r: I::Region) -> I::Region {
        r
    }

    fn fold_const(&mut self, c: I::Const) -> I::Const {
        c.super_fold_with(self)
    }

    fn fold_predicate(&mut self, p: I::Predicate) -> I::Predicate {
        p.super_fold_with(self)
    }

    fn fold_clauses(&mut self, c: I::Clauses) -> I::Clauses {
        c.super_fold_with(self)
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=cx | COMPLEXITY=16 | LINES=42

```rust
/// This trait is implemented for every folding traversal. There is a fold
/// method defined for every type of interest. Each such method has a default
/// that does an "identity" fold.
///
/// A blanket implementation of this trait (that defers to the relevant
/// method of [`TypeFolder`]) is provided for all infallible folders in
/// order to ensure the two APIs are coherent.
pub trait FallibleTypeFolder<I: Interner>: Sized {
    type Error;

    fn cx(&self) -> I;

    fn try_fold_binder<T>(&mut self, t: ty::Binder<I, T>) -> Result<ty::Binder<I, T>, Self::Error>
    where
        T: TypeFoldable<I>,
    {
        t.try_super_fold_with(self)
    }

    fn try_fold_ty(&mut self, t: I::Ty) -> Result<I::Ty, Self::Error> {
        t.try_super_fold_with(self)
    }

    // The default region folder is a no-op because `Region` is non-recursive
    // and has no `super_fold_with` method to call.
    fn try_fold_region(&mut self, r: I::Region) -> Result<I::Region, Self::Error> {
        Ok(r)
    }

    fn try_fold_const(&mut self, c: I::Const) -> Result<I::Const, Self::Error> {
        c.try_super_fold_with(self)
    }

    fn try_fold_predicate(&mut self, p: I::Predicate) -> Result<I::Predicate, Self::Error> {
        p.try_super_fold_with(self)
    }

    fn try_fold_clauses(&mut self, c: I::Clauses) -> Result<I::Clauses, Self::Error> {
        c.try_super_fold_with(self)
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=try_fold_with | COMPLEXITY=7 | LINES=13

```rust
///////////////////////////////////////////////////////////////////////////
// Traversal implementations.

impl<I: Interner, T: TypeFoldable<I>, U: TypeFoldable<I>> TypeFoldable<I> for (T, U) {
    fn try_fold_with<F: FallibleTypeFolder<I>>(self, folder: &mut F) -> Result<(T, U), F::Error> {
        Ok((self.0.try_fold_with(folder)?, self.1.try_fold_with(folder)?))
    }

    fn fold_with<F: TypeFolder<I>>(self, folder: &mut F) -> Self {
        (self.0.fold_with(folder), self.1.fold_with(folder))
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=try_fold_with | COMPLEXITY=7 | LINES=19

```rust
impl<I: Interner, A: TypeFoldable<I>, B: TypeFoldable<I>, C: TypeFoldable<I>> TypeFoldable<I>
    for (A, B, C)
{
    fn try_fold_with<F: FallibleTypeFolder<I>>(
        self,
        folder: &mut F,
    ) -> Result<(A, B, C), F::Error> {
        Ok((
            self.0.try_fold_with(folder)?,
            self.1.try_fold_with(folder)?,
            self.2.try_fold_with(folder)?,
        ))
    }

    fn fold_with<F: TypeFolder<I>>(self, folder: &mut F) -> Self {
        (self.0.fold_with(folder), self.1.fold_with(folder), self.2.fold_with(folder))
    }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=try_fold_with | COMPLEXITY=10 | LINES=13

```rust
impl<I: Interner, T: TypeFoldable<I>> TypeFoldable<I> for Option<T> {
    fn try_fold_with<F: FallibleTypeFolder<I>>(self, folder: &mut F) -> Result<Self, F::Error> {
        Ok(match self {
            Some(v) => Some(v.try_fold_with(folder)?),
            None => None,
        })
    }

    fn fold_with<F: TypeFolder<I>>(self, folder: &mut F) -> Self {
        Some(self?.fold_with(folder))
    }
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=try_fold_with | COMPLEXITY=15 | LINES=16

```rust
impl<I: Interner, T: TypeFoldable<I>, E: TypeFoldable<I>> TypeFoldable<I> for Result<T, E> {
    fn try_fold_with<F: FallibleTypeFolder<I>>(self, folder: &mut F) -> Result<Self, F::Error> {
        Ok(match self {
            Ok(v) => Ok(v.try_fold_with(folder)?),
            Err(e) => Err(e.try_fold_with(folder)?),
        })
    }

    fn fold_with<F: TypeFolder<I>>(self, folder: &mut F) -> Self {
        match self {
            Ok(v) => Ok(v.fold_with(folder)),
            Err(e) => Err(e.fold_with(folder)),
        }
    }
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=fold_arc | COMPLEXITY=14 | LINES=38

```rust
fn fold_arc<T: Clone, E>(
    mut arc: Arc<T>,
    fold: impl FnOnce(T) -> Result<T, E>,
) -> Result<Arc<T>, E> {
    // We merely want to replace the contained `T`, if at all possible,
    // so that we don't needlessly allocate a new `Arc` or indeed clone
    // the contained type.
    unsafe {
        // First step is to ensure that we have a unique reference to
        // the contained type, which `Arc::make_mut` will accomplish (by
        // allocating a new `Arc` and cloning the `T` only if required).
        // This is done *before* casting to `Arc<ManuallyDrop<T>>` so that
        // panicking during `make_mut` does not leak the `T`.
        Arc::make_mut(&mut arc);

        // Casting to `Arc<ManuallyDrop<T>>` is safe because `ManuallyDrop`
        // is `repr(transparent)`.
        let ptr = Arc::into_raw(arc).cast::<mem::ManuallyDrop<T>>();
        let mut unique = Arc::from_raw(ptr);

        // Call to `Arc::make_mut` above guarantees that `unique` is the
        // sole reference to the contained value, so we can avoid doing
        // a checked `get_mut` here.
        let slot = Arc::get_mut(&mut unique).unwrap_unchecked();

        // Semantically move the contained type out from `unique`, fold
        // it, then move the folded value back into `unique`. Should
        // folding fail, `ManuallyDrop` ensures that the "moved-out"
        // value is not re-dropped.
        let owned = mem::ManuallyDrop::take(slot);
        let folded = fold(owned)?;
        *slot = mem::ManuallyDrop::new(folded);

        // Cast back to `Arc<T>`.
        Ok(Arc::from_raw(Arc::into_raw(unique).cast()))
    }
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=try_fold_with | COMPLEXITY=10 | LINES=12

```rust
impl<I: Interner, T: TypeFoldable<I>> TypeFoldable<I> for Arc<T> {
    fn try_fold_with<F: FallibleTypeFolder<I>>(self, folder: &mut F) -> Result<Self, F::Error> {
        fold_arc(self, |t| t.try_fold_with(folder))
    }

    fn fold_with<F: TypeFolder<I>>(self, folder: &mut F) -> Self {
        match fold_arc::<T, Infallible>(self, |t| Ok(t.fold_with(folder))) {
            Ok(t) => t,
        }
    }
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=try_fold_with | COMPLEXITY=6 | LINES=12

```rust
impl<I: Interner, T: TypeFoldable<I>> TypeFoldable<I> for Box<T> {
    fn try_fold_with<F: FallibleTypeFolder<I>>(mut self, folder: &mut F) -> Result<Self, F::Error> {
        *self = (*self).try_fold_with(folder)?;
        Ok(self)
    }

    fn fold_with<F: TypeFolder<I>>(mut self, folder: &mut F) -> Self {
        *self = (*self).fold_with(folder);
        self
    }
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=try_fold_with | COMPLEXITY=6 | LINES=10

```rust
impl<I: Interner, T: TypeFoldable<I>> TypeFoldable<I> for Vec<T> {
    fn try_fold_with<F: FallibleTypeFolder<I>>(self, folder: &mut F) -> Result<Self, F::Error> {
        self.into_iter().map(|t| t.try_fold_with(folder)).collect()
    }

    fn fold_with<F: TypeFolder<I>>(self, folder: &mut F) -> Self {
        self.into_iter().map(|t| t.fold_with(folder)).collect()
    }
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=try_fold_with | COMPLEXITY=6 | LINES=10

```rust
impl<I: Interner, T: TypeFoldable<I>> TypeFoldable<I> for ThinVec<T> {
    fn try_fold_with<F: FallibleTypeFolder<I>>(self, folder: &mut F) -> Result<Self, F::Error> {
        self.into_iter().map(|t| t.try_fold_with(folder)).collect()
    }

    fn fold_with<F: TypeFolder<I>>(self, folder: &mut F) -> Self {
        self.into_iter().map(|t| t.fold_with(folder)).collect()
    }
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=try_fold_with | COMPLEXITY=6 | LINES=10

```rust
impl<I: Interner, T: TypeFoldable<I>> TypeFoldable<I> for Box<[T]> {
    fn try_fold_with<F: FallibleTypeFolder<I>>(self, folder: &mut F) -> Result<Self, F::Error> {
        Vec::from(self).try_fold_with(folder).map(Vec::into_boxed_slice)
    }

    fn fold_with<F: TypeFolder<I>>(self, folder: &mut F) -> Self {
        Vec::into_boxed_slice(Vec::from(self).fold_with(folder))
    }
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=try_fold_with | COMPLEXITY=6 | LINES=10

```rust
impl<I: Interner, T: TypeFoldable<I>, Ix: Idx> TypeFoldable<I> for IndexVec<Ix, T> {
    fn try_fold_with<F: FallibleTypeFolder<I>>(self, folder: &mut F) -> Result<Self, F::Error> {
        self.raw.try_fold_with(folder).map(IndexVec::from_raw)
    }

    fn fold_with<F: TypeFolder<I>>(self, folder: &mut F) -> Self {
        IndexVec::from_raw(self.raw.fold_with(folder))
    }
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=STRUCT | NAME=Shifter | COMPLEXITY=5 | LINES=15

```rust
///////////////////////////////////////////////////////////////////////////
// Shifter
//
// Shifts the De Bruijn indices on all escaping bound vars by a
// fixed amount. Useful in instantiation or when otherwise introducing
// a binding level that is not intended to capture the existing bound
// vars. See comment on `shift_vars_through_binders` method in
// `rustc_middle/src/ty/generic_args.rs` for more details.

struct Shifter<I: Interner> {
    cx: I,
    current_index: ty::DebruijnIndex,
    amount: u32,
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=new | COMPLEXITY=4 | LINES=6

```rust
impl<I: Interner> Shifter<I> {
    fn new(cx: I, amount: u32) -> Self {
        Shifter { cx, current_index: ty::INNERMOST, amount }
    }
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=cx | COMPLEXITY=40 | LINES=49

```rust
impl<I: Interner> TypeFolder<I> for Shifter<I> {
    fn cx(&self) -> I {
        self.cx
    }

    fn fold_binder<T: TypeFoldable<I>>(&mut self, t: ty::Binder<I, T>) -> ty::Binder<I, T> {
        self.current_index.shift_in(1);
        let t = t.super_fold_with(self);
        self.current_index.shift_out(1);
        t
    }

    fn fold_region(&mut self, r: I::Region) -> I::Region {
        match r.kind() {
            ty::ReBound(debruijn, br) if debruijn >= self.current_index => {
                let debruijn = debruijn.shifted_in(self.amount);
                Region::new_bound(self.cx, debruijn, br)
            }
            _ => r,
        }
    }

    fn fold_ty(&mut self, ty: I::Ty) -> I::Ty {
        match ty.kind() {
            ty::Bound(debruijn, bound_ty) if debruijn >= self.current_index => {
                let debruijn = debruijn.shifted_in(self.amount);
                Ty::new_bound(self.cx, debruijn, bound_ty)
            }

            _ if ty.has_vars_bound_at_or_above(self.current_index) => ty.super_fold_with(self),
            _ => ty,
        }
    }

    fn fold_const(&mut self, ct: I::Const) -> I::Const {
        match ct.kind() {
            ty::ConstKind::Bound(debruijn, bound_ct) if debruijn >= self.current_index => {
                let debruijn = debruijn.shifted_in(self.amount);
                Const::new_bound(self.cx, debruijn, bound_ct)
            }
            _ => ct.super_fold_with(self),
        }
    }

    fn fold_predicate(&mut self, p: I::Predicate) -> I::Predicate {
        if p.has_vars_bound_at_or_above(self.current_index) { p.super_fold_with(self) } else { p }
    }
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=FUNCTION | NAME=shift_region | COMPLEXITY=9 | LINES=9

```rust
pub fn shift_region<I: Interner>(cx: I, region: I::Region, amount: u32) -> I::Region {
    match region.kind() {
        ty::ReBound(debruijn, br) if amount > 0 => {
            Region::new_bound(cx, debruijn.shifted_in(amount), br)
        }
        _ => region,
    }
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=FUNCTION | NAME=shift_vars | COMPLEXITY=6 | LINES=12

```rust
#[instrument(level = "trace", skip(cx), ret)]
pub fn shift_vars<I: Interner, T>(cx: I, value: T, amount: u32) -> T
where
    T: TypeFoldable<I>,
{
    if amount == 0 || !value.has_escaping_bound_vars() {
        value
    } else {
        value.fold_with(&mut Shifter::new(cx, amount))
    }
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=FUNCTION | NAME=fold_regions | COMPLEXITY=2 | LINES=14

```rust
///////////////////////////////////////////////////////////////////////////
// Region folder

pub fn fold_regions<I: Interner, T>(
    cx: I,
    value: T,
    f: impl FnMut(I::Region, ty::DebruijnIndex) -> I::Region,
) -> T
where
    T: TypeFoldable<I>,
{
    value.fold_with(&mut RegionFolder::new(cx, f))
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=STRUCT | NAME=RegionFolder | COMPLEXITY=5 | LINES=21

```rust
/// Folds over the substructure of a type, visiting its component
/// types and all regions that occur *free* within it.
///
/// That is, function pointer types and trait object can introduce
/// new bound regions which are not visited by this visitors as
/// they are not free; only regions that occur free will be
/// visited by `fld_r`.
pub struct RegionFolder<I, F> {
    cx: I,

    /// Stores the index of a binder *just outside* the stuff we have
    /// visited. So this begins as INNERMOST; when we pass through a
    /// binder, it is incremented (via `shift_in`).
    current_index: ty::DebruijnIndex,

    /// Callback invokes for each free region. The `DebruijnIndex`
    /// points to the binder *just outside* the ones we have passed
    /// through.
    fold_region_fn: F,
}
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=FUNCTION | NAME=new | COMPLEXITY=4 | LINES=7

```rust
impl<I, F> RegionFolder<I, F> {
    #[inline]
    pub fn new(cx: I, fold_region_fn: F) -> RegionFolder<I, F> {
        RegionFolder { cx, current_index: ty::INNERMOST, fold_region_fn }
    }
}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=FUNCTION | NAME=cx | COMPLEXITY=33 | LINES=61

```rust
impl<I, F> TypeFolder<I> for RegionFolder<I, F>
where
    I: Interner,
    F: FnMut(I::Region, ty::DebruijnIndex) -> I::Region,
{
    fn cx(&self) -> I {
        self.cx
    }

    fn fold_binder<T: TypeFoldable<I>>(&mut self, t: ty::Binder<I, T>) -> ty::Binder<I, T> {
        self.current_index.shift_in(1);
        let t = t.super_fold_with(self);
        self.current_index.shift_out(1);
        t
    }

    #[instrument(skip(self), level = "debug", ret)]
    fn fold_region(&mut self, r: I::Region) -> I::Region {
        match r.kind() {
            ty::ReBound(debruijn, _) if debruijn < self.current_index => {
                debug!(?self.current_index, "skipped bound region");
                r
            }
            _ => {
                debug!(?self.current_index, "folding free region");
                (self.fold_region_fn)(r, self.current_index)
            }
        }
    }

    fn fold_ty(&mut self, t: I::Ty) -> I::Ty {
        if t.has_type_flags(
            TypeFlags::HAS_FREE_REGIONS | TypeFlags::HAS_RE_BOUND | TypeFlags::HAS_RE_ERASED,
        ) {
            t.super_fold_with(self)
        } else {
            t
        }
    }

    fn fold_const(&mut self, ct: I::Const) -> I::Const {
        if ct.has_type_flags(
            TypeFlags::HAS_FREE_REGIONS | TypeFlags::HAS_RE_BOUND | TypeFlags::HAS_RE_ERASED,
        ) {
            ct.super_fold_with(self)
        } else {
            ct
        }
    }

    fn fold_predicate(&mut self, p: I::Predicate) -> I::Predicate {
        if p.has_type_flags(
            TypeFlags::HAS_FREE_REGIONS | TypeFlags::HAS_RE_BOUND | TypeFlags::HAS_RE_ERASED,
        ) {
            p.super_fold_with(self)
        } else {
            p
        }
    }
}
```

---
*Generated by AST tracing system*
