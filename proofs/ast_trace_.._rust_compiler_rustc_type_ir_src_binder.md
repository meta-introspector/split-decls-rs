# AST Trace: ../rust/compiler/rustc_type_ir/src/binder.rs

Generated 48 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use std::marker::PhantomData;
use std::ops::{ControlFlow, Deref};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4

```rust
use derive_where::derive_where;
#[cfg(feature = "nightly")]
use rustc_macros::{Decodable_NoContext, Encodable_NoContext, HashStable_NoContext};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4

```rust
use tracing::instrument;

use crate::data_structures::SsoHashSet;
use crate::fold::{FallibleTypeFolder, TypeFoldable, TypeFolder, TypeSuperFoldable};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use crate::inherent::*;
use crate::lift::Lift;
use crate::visit::{Flags, TypeSuperVisitable, TypeVisitable, TypeVisitableExt, TypeVisitor};
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::{self as ty, Interner};
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=STRUCT | NAME=Binder | COMPLEXITY=11 | LINES=16

```rust
/// `Binder` is a binder for higher-ranked lifetimes or types. It is part of the
/// compiler's representation for things like `for<'a> Fn(&'a isize)`
/// (which would be represented by the type `PolyTraitRef == Binder<I, TraitRef>`).
///
/// See <https://rustc-dev-guide.rust-lang.org/ty_module/instantiating_binders.html>
/// for more details.
///
/// `Decodable` and `Encodable` are implemented for `Binder<T>` using the `impl_binder_encode_decode!` macro.
#[derive_where(Clone, Hash, PartialEq, Debug; I: Interner, T)]
#[derive_where(Copy; I: Interner, T: Copy)]
#[cfg_attr(feature = "nightly", derive(HashStable_NoContext))]
pub struct Binder<I: Interner, T> {
    value: T,
    bound_vars: I::BoundVarKinds,
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=2

```rust
impl<I: Interner, T: Eq> Eq for Binder<I, T> {}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=lift_to_interner | COMPLEXITY=7 | LINES=17

```rust
// FIXME: We manually derive `Lift` because the `derive(Lift_Generic)` doesn't
// understand how to turn `T` to `T::Lifted` in the output `type Lifted`.
impl<I: Interner, U: Interner, T> Lift<U> for Binder<I, T>
where
    T: Lift<U>,
    I::BoundVarKinds: Lift<U, Lifted = U::BoundVarKinds>,
{
    type Lifted = Binder<U, T::Lifted>;

    fn lift_to_interner(self, cx: U) -> Option<Self::Lifted> {
        Some(Binder {
            value: self.value.lift_to_interner(cx)?,
            bound_vars: self.bound_vars.lift_to_interner(cx)?,
        })
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=18 | LINES=28

```rust
#[cfg(feature = "nightly")]
macro_rules! impl_binder_encode_decode {
    ($($t:ty),+ $(,)?) => {
        $(
            impl<I: Interner, E: crate::rustc_serialize::Encoder> crate::rustc_serialize::Encodable<E> for ty::Binder<I, $t>
            where
                $t: crate::rustc_serialize::Encodable<E>,
                I::BoundVarKinds: crate::rustc_serialize::Encodable<E>,
            {
                fn encode(&self, e: &mut E) {
                    self.bound_vars().encode(e);
                    self.as_ref().skip_binder().encode(e);
                }
            }
            impl<I: Interner, D: crate::rustc_serialize::Decoder> crate::rustc_serialize::Decodable<D> for ty::Binder<I, $t>
            where
                $t: TypeVisitable<I> + crate::rustc_serialize::Decodable<D>,
                I::BoundVarKinds: crate::rustc_serialize::Decodable<D>,
            {
                fn decode(decoder: &mut D) -> Self {
                    let bound_vars = crate::rustc_serialize::Decodable::decode(decoder);
                    ty::Binder::bind_with_vars(crate::rustc_serialize::Decodable::decode(decoder), bound_vars)
                }
            }
        )*
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=11

```rust
#[cfg(feature = "nightly")]
impl_binder_encode_decode! {
    ty::FnSig<I>,
    ty::FnSigTys<I>,
    ty::TraitPredicate<I>,
    ty::ExistentialPredicate<I>,
    ty::TraitRef<I>,
    ty::ExistentialTraitRef<I>,
    ty::HostEffectPredicate<I>,
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=dummy | COMPLEXITY=11 | LINES=26

```rust
impl<I: Interner, T> Binder<I, T>
where
    T: TypeVisitable<I>,
{
    /// Wraps `value` in a binder, asserting that `value` does not
    /// contain any bound vars that would be bound by the
    /// binder. This is commonly used to 'inject' a value T into a
    /// different binding level.
    #[track_caller]
    pub fn dummy(value: T) -> Binder<I, T> {
        assert!(
            !value.has_escaping_bound_vars(),
            "`{value:?}` has escaping bound vars, so it cannot be wrapped in a dummy binder."
        );
        Binder { value, bound_vars: Default::default() }
    }

    pub fn bind_with_vars(value: T, bound_vars: I::BoundVarKinds) -> Binder<I, T> {
        if cfg!(debug_assertions) {
            let mut validator = ValidateBoundVars::new(bound_vars);
            let _ = value.visit_with(&mut validator);
        }
        Binder { value, bound_vars }
    }
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=try_fold_with | COMPLEXITY=6 | LINES=10

```rust
impl<I: Interner, T: TypeFoldable<I>> TypeFoldable<I> for Binder<I, T> {
    fn try_fold_with<F: FallibleTypeFolder<I>>(self, folder: &mut F) -> Result<Self, F::Error> {
        folder.try_fold_binder(self)
    }

    fn fold_with<F: TypeFolder<I>>(self, folder: &mut F) -> Self {
        folder.fold_binder(self)
    }
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=visit_with | COMPLEXITY=5 | LINES=6

```rust
impl<I: Interner, T: TypeVisitable<I>> TypeVisitable<I> for Binder<I, T> {
    fn visit_with<V: TypeVisitor<I>>(&self, visitor: &mut V) -> V::Result {
        visitor.visit_binder(self)
    }
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=try_super_fold_with | COMPLEXITY=6 | LINES=13

```rust
impl<I: Interner, T: TypeFoldable<I>> TypeSuperFoldable<I> for Binder<I, T> {
    fn try_super_fold_with<F: FallibleTypeFolder<I>>(
        self,
        folder: &mut F,
    ) -> Result<Self, F::Error> {
        self.try_map_bound(|t| t.try_fold_with(folder))
    }

    fn super_fold_with<F: TypeFolder<I>>(self, folder: &mut F) -> Self {
        self.map_bound(|t| t.fold_with(folder))
    }
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=super_visit_with | COMPLEXITY=5 | LINES=6

```rust
impl<I: Interner, T: TypeVisitable<I>> TypeSuperVisitable<I> for Binder<I, T> {
    fn super_visit_with<V: TypeVisitor<I>>(&self, visitor: &mut V) -> V::Result {
        self.as_ref().skip_binder().visit_with(visitor)
    }
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=skip_binder | COMPLEXITY=42 | LINES=101

```rust
impl<I: Interner, T> Binder<I, T> {
    /// Returns the value contained inside of this `for<'a>`. Accessing generic args
    /// in the returned value is generally incorrect.
    ///
    /// Please read <https://rustc-dev-guide.rust-lang.org/ty_module/instantiating_binders.html>
    /// before using this function. It is usually better to discharge the binder using
    /// `no_bound_vars` or `instantiate_bound_regions` or something like that.
    ///
    /// `skip_binder` is only valid when you are either extracting data that does not reference
    /// any generic arguments, e.g. a `DefId`, or when you're making sure you only pass the
    /// value to things which can handle escaping bound vars.
    ///
    /// See existing uses of `.skip_binder()` in `crate::rustc_trait_selection::traits::select`
    /// or `rustc_next_trait_solver` for examples.
    pub fn skip_binder(self) -> T {
        self.value
    }

    pub fn bound_vars(&self) -> I::BoundVarKinds {
        self.bound_vars
    }

    pub fn as_ref(&self) -> Binder<I, &T> {
        Binder { value: &self.value, bound_vars: self.bound_vars }
    }

    pub fn as_deref(&self) -> Binder<I, &T::Target>
    where
        T: Deref,
    {
        Binder { value: &self.value, bound_vars: self.bound_vars }
    }

    pub fn map_bound_ref<F, U: TypeVisitable<I>>(&self, f: F) -> Binder<I, U>
    where
        F: FnOnce(&T) -> U,
    {
        self.as_ref().map_bound(f)
    }

    pub fn map_bound<F, U: TypeVisitable<I>>(self, f: F) -> Binder<I, U>
    where
        F: FnOnce(T) -> U,
    {
        let Binder { value, bound_vars } = self;
        let value = f(value);
        if cfg!(debug_assertions) {
            let mut validator = ValidateBoundVars::new(bound_vars);
            let _ = value.visit_with(&mut validator);
        }
        Binder { value, bound_vars }
    }

    pub fn try_map_bound<F, U: TypeVisitable<I>, E>(self, f: F) -> Result<Binder<I, U>, E>
    where
        F: FnOnce(T) -> Result<U, E>,
    {
        let Binder { value, bound_vars } = self;
        let value = f(value)?;
        if cfg!(debug_assertions) {
            let mut validator = ValidateBoundVars::new(bound_vars);
            let _ = value.visit_with(&mut validator);
        }
        Ok(Binder { value, bound_vars })
    }

    /// Wraps a `value` in a binder, using the same bound variables as the
    /// current `Binder`. This should not be used if the new value *changes*
    /// the bound variables. Note: the (old or new) value itself does not
    /// necessarily need to *name* all the bound variables.
    ///
    /// This currently doesn't do anything different than `bind`, because we
    /// don't actually track bound vars. However, semantically, it is different
    /// because bound vars aren't allowed to change here, whereas they are
    /// in `bind`. This may be (debug) asserted in the future.
    pub fn rebind<U>(&self, value: U) -> Binder<I, U>
    where
        U: TypeVisitable<I>,
    {
        Binder::bind_with_vars(value, self.bound_vars)
    }

    /// Unwraps and returns the value within, but only if it contains
    /// no bound vars at all. (In other words, if this binder --
    /// and indeed any enclosing binder -- doesn't bind anything at
    /// all.) Otherwise, returns `None`.
    ///
    /// (One could imagine having a method that just unwraps a single
    /// binder, but permits late-bound vars bound by enclosing
    /// binders, but that would require adjusting the debruijn
    /// indices, and given the shallow binding structure we often use,
    /// would not be that useful.)
    pub fn no_bound_vars(self) -> Option<T>
    where
        T: TypeVisitable<I>,
    {
        // `self.value` is equivalent to `self.skip_binder()`
        if self.value.has_escaping_bound_vars() { None } else { Some(self.skip_binder()) }
    }
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=transpose | COMPLEXITY=5 | LINES=7

```rust
impl<I: Interner, T> Binder<I, Option<T>> {
    pub fn transpose(self) -> Option<Binder<I, T>> {
        let Binder { value, bound_vars } = self;
        value.map(|value| Binder { value, bound_vars })
    }
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=iter | COMPLEXITY=5 | LINES=7

```rust
impl<I: Interner, T: IntoIterator> Binder<I, T> {
    pub fn iter(self) -> impl Iterator<Item = Binder<I, T::Item>> {
        let Binder { value, bound_vars } = self;
        value.into_iter().map(move |value| Binder { value, bound_vars })
    }
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=STRUCT | NAME=ValidateBoundVars | COMPLEXITY=2 | LINES=9

```rust
pub struct ValidateBoundVars<I: Interner> {
    bound_vars: I::BoundVarKinds,
    binder_index: ty::DebruijnIndex,
    // We only cache types because any complex const will have to step through
    // a type at some point anyways. We may encounter the same variable at
    // different levels of binding, so this can't just be `Ty`.
    visited: SsoHashSet<(ty::DebruijnIndex, I::Ty)>,
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=new | COMPLEXITY=4 | LINES=10

```rust
impl<I: Interner> ValidateBoundVars<I> {
    pub fn new(bound_vars: I::BoundVarKinds) -> Self {
        ValidateBoundVars {
            bound_vars,
            binder_index: ty::INNERMOST,
            visited: SsoHashSet::default(),
        }
    }
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=visit_binder | COMPLEXITY=56 | LINES=65

```rust
impl<I: Interner> TypeVisitor<I> for ValidateBoundVars<I> {
    type Result = ControlFlow<()>;

    fn visit_binder<T: TypeVisitable<I>>(&mut self, t: &Binder<I, T>) -> Self::Result {
        self.binder_index.shift_in(1);
        let result = t.super_visit_with(self);
        self.binder_index.shift_out(1);
        result
    }

    fn visit_ty(&mut self, t: I::Ty) -> Self::Result {
        if t.outer_exclusive_binder() < self.binder_index
            || !self.visited.insert((self.binder_index, t))
        {
            return ControlFlow::Break(());
        }
        match t.kind() {
            ty::Bound(debruijn, bound_ty) if debruijn == self.binder_index => {
                let idx = bound_ty.var().as_usize();
                if self.bound_vars.len() <= idx {
                    panic!("Not enough bound vars: {:?} not found in {:?}", t, self.bound_vars);
                }
                bound_ty.assert_eq(self.bound_vars.get(idx).unwrap());
            }
            _ => {}
        };

        t.super_visit_with(self)
    }

    fn visit_const(&mut self, c: I::Const) -> Self::Result {
        if c.outer_exclusive_binder() < self.binder_index {
            return ControlFlow::Break(());
        }
        match c.kind() {
            ty::ConstKind::Bound(debruijn, bound_const) if debruijn == self.binder_index => {
                let idx = bound_const.var().as_usize();
                if self.bound_vars.len() <= idx {
                    panic!("Not enough bound vars: {:?} not found in {:?}", c, self.bound_vars);
                }
                bound_const.assert_eq(self.bound_vars.get(idx).unwrap());
            }
            _ => {}
        };

        c.super_visit_with(self)
    }

    fn visit_region(&mut self, r: I::Region) -> Self::Result {
        match r.kind() {
            ty::ReBound(index, br) if index == self.binder_index => {
                let idx = br.var().as_usize();
                if self.bound_vars.len() <= idx {
                    panic!("Not enough bound vars: {:?} not found in {:?}", r, self.bound_vars);
                }
                br.assert_eq(self.bound_vars.get(idx).unwrap());
            }

            _ => (),
        };

        ControlFlow::Continue(())
    }
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=STRUCT | NAME=EarlyBinder | COMPLEXITY=5 | LINES=18

```rust
/// Similar to [`Binder`] except that it tracks early bound generics, i.e. `struct Foo<T>(T)`
/// needs `T` instantiated immediately. This type primarily exists to avoid forgetting to call
/// `instantiate`.
///
/// See <https://rustc-dev-guide.rust-lang.org/ty_module/early_binder.html> for more details.
#[derive_where(Clone, PartialEq, Ord, Hash, Debug; I: Interner, T)]
#[derive_where(PartialOrd; I: Interner, T: Ord)]
#[derive_where(Copy; I: Interner, T: Copy)]
#[cfg_attr(
    feature = "nightly",
    derive(Encodable_NoContext, Decodable_NoContext, HashStable_NoContext)
)]
pub struct EarlyBinder<I: Interner, T> {
    value: T,
    #[derive_where(skip(Debug))]
    _tcx: PhantomData<fn() -> I>,
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=2

```rust
impl<I: Interner, T: Eq> Eq for EarlyBinder<I, T> {}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=4

```rust
/// For early binders, you should first call `instantiate` before using any visitors.
#[cfg(feature = "nightly")]
impl<I: Interner, T> !TypeFoldable<I> for ty::EarlyBinder<I, T> {}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=4

```rust
/// For early binders, you should first call `instantiate` before using any visitors.
#[cfg(feature = "nightly")]
impl<I: Interner, T> !TypeVisitable<I> for ty::EarlyBinder<I, T> {}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=FUNCTION | NAME=bind | COMPLEXITY=19 | LINES=57

```rust
impl<I: Interner, T> EarlyBinder<I, T> {
    pub fn bind(value: T) -> EarlyBinder<I, T> {
        EarlyBinder { value, _tcx: PhantomData }
    }

    pub fn as_ref(&self) -> EarlyBinder<I, &T> {
        EarlyBinder { value: &self.value, _tcx: PhantomData }
    }

    pub fn map_bound_ref<F, U>(&self, f: F) -> EarlyBinder<I, U>
    where
        F: FnOnce(&T) -> U,
    {
        self.as_ref().map_bound(f)
    }

    pub fn map_bound<F, U>(self, f: F) -> EarlyBinder<I, U>
    where
        F: FnOnce(T) -> U,
    {
        let value = f(self.value);
        EarlyBinder { value, _tcx: PhantomData }
    }

    pub fn try_map_bound<F, U, E>(self, f: F) -> Result<EarlyBinder<I, U>, E>
    where
        F: FnOnce(T) -> Result<U, E>,
    {
        let value = f(self.value)?;
        Ok(EarlyBinder { value, _tcx: PhantomData })
    }

    pub fn rebind<U>(&self, value: U) -> EarlyBinder<I, U> {
        EarlyBinder { value, _tcx: PhantomData }
    }

    /// Skips the binder and returns the "bound" value. Accessing generic args
    /// in the returned value is generally incorrect.
    ///
    /// Please read <https://rustc-dev-guide.rust-lang.org/ty_module/early_binder.html>
    /// before using this function.
    ///
    /// Only use this to extract data that does not depend on generic parameters, e.g.
    /// to get the `DefId` of the inner value or the number of arguments ofan `FnSig`,
    /// or while making sure to only pass the value to functions which are explicitly
    /// set up to handle these uninstantiated generic parameters.
    ///
    /// To skip the binder on `x: &EarlyBinder<I, T>` to obtain `&T`, leverage
    /// [`EarlyBinder::as_ref`](EarlyBinder::as_ref): `x.as_ref().skip_binder()`.
    ///
    /// See also [`Binder::skip_binder`](Binder::skip_binder), which is
    /// the analogous operation on [`Binder`].
    pub fn skip_binder(self) -> T {
        self.value
    }
}
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=FUNCTION | NAME=transpose | COMPLEXITY=4 | LINES=6

```rust
impl<I: Interner, T> EarlyBinder<I, Option<T>> {
    pub fn transpose(self) -> Option<EarlyBinder<I, T>> {
        self.value.map(|value| EarlyBinder { value, _tcx: PhantomData })
    }
}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=FUNCTION | NAME=iter_instantiated | COMPLEXITY=6 | LINES=18

```rust
impl<I: Interner, Iter: IntoIterator> EarlyBinder<I, Iter>
where
    Iter::Item: TypeFoldable<I>,
{
    pub fn iter_instantiated<A>(self, cx: I, args: A) -> IterInstantiated<I, Iter, A>
    where
        A: SliceLike<Item = I::GenericArg>,
    {
        IterInstantiated { it: self.value.into_iter(), cx, args }
    }

    /// Similar to [`instantiate_identity`](EarlyBinder::instantiate_identity),
    /// but on an iterator of `TypeFoldable` values.
    pub fn iter_identity(self) -> Iter::IntoIter {
        self.value.into_iter()
    }
}
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=STRUCT | NAME=IterInstantiated | COMPLEXITY=2 | LINES=6

```rust
pub struct IterInstantiated<I: Interner, Iter: IntoIterator, A> {
    it: Iter::IntoIter,
    cx: I,
    args: A,
}
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=FUNCTION | NAME=next | COMPLEXITY=7 | LINES=19

```rust
impl<I: Interner, Iter: IntoIterator, A> Iterator for IterInstantiated<I, Iter, A>
where
    Iter::Item: TypeFoldable<I>,
    A: SliceLike<Item = I::GenericArg>,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        Some(
            EarlyBinder { value: self.it.next()?, _tcx: PhantomData }
                .instantiate(self.cx, self.args),
        )
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.it.size_hint()
    }
}
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=FUNCTION | NAME=next_back | COMPLEXITY=6 | LINES=14

```rust
impl<I: Interner, Iter: IntoIterator, A> DoubleEndedIterator for IterInstantiated<I, Iter, A>
where
    Iter::IntoIter: DoubleEndedIterator,
    Iter::Item: TypeFoldable<I>,
    A: SliceLike<Item = I::GenericArg>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        Some(
            EarlyBinder { value: self.it.next_back()?, _tcx: PhantomData }
                .instantiate(self.cx, self.args),
        )
    }
}
```

## Block 32
**Metadata**: AST_ID=32 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=8

```rust
impl<I: Interner, Iter: IntoIterator, A> ExactSizeIterator for IterInstantiated<I, Iter, A>
where
    Iter::IntoIter: ExactSizeIterator,
    Iter::Item: TypeFoldable<I>,
    A: SliceLike<Item = I::GenericArg>,
{
}
```

## Block 33
**Metadata**: AST_ID=33 | TYPE=FUNCTION | NAME=iter_instantiated_copied | COMPLEXITY=7 | LINES=20

```rust
impl<'s, I: Interner, Iter: IntoIterator> EarlyBinder<I, Iter>
where
    Iter::Item: Deref,
    <Iter::Item as Deref>::Target: Copy + TypeFoldable<I>,
{
    pub fn iter_instantiated_copied(
        self,
        cx: I,
        args: &'s [I::GenericArg],
    ) -> IterInstantiatedCopied<'s, I, Iter> {
        IterInstantiatedCopied { it: self.value.into_iter(), cx, args }
    }

    /// Similar to [`instantiate_identity`](EarlyBinder::instantiate_identity),
    /// but on an iterator of values that deref to a `TypeFoldable`.
    pub fn iter_identity_copied(self) -> IterIdentityCopied<Iter> {
        IterIdentityCopied { it: self.value.into_iter() }
    }
}
```

## Block 34
**Metadata**: AST_ID=34 | TYPE=STRUCT | NAME=IterInstantiatedCopied | COMPLEXITY=2 | LINES=6

```rust
pub struct IterInstantiatedCopied<'a, I: Interner, Iter: IntoIterator> {
    it: Iter::IntoIter,
    cx: I,
    args: &'a [I::GenericArg],
}
```

## Block 35
**Metadata**: AST_ID=35 | TYPE=FUNCTION | NAME=next | COMPLEXITY=9 | LINES=18

```rust
impl<I: Interner, Iter: IntoIterator> Iterator for IterInstantiatedCopied<'_, I, Iter>
where
    Iter::Item: Deref,
    <Iter::Item as Deref>::Target: Copy + TypeFoldable<I>,
{
    type Item = <Iter::Item as Deref>::Target;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().map(|value| {
            EarlyBinder { value: *value, _tcx: PhantomData }.instantiate(self.cx, self.args)
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.it.size_hint()
    }
}
```

## Block 36
**Metadata**: AST_ID=36 | TYPE=FUNCTION | NAME=next_back | COMPLEXITY=7 | LINES=13

```rust
impl<I: Interner, Iter: IntoIterator> DoubleEndedIterator for IterInstantiatedCopied<'_, I, Iter>
where
    Iter::IntoIter: DoubleEndedIterator,
    Iter::Item: Deref,
    <Iter::Item as Deref>::Target: Copy + TypeFoldable<I>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.it.next_back().map(|value| {
            EarlyBinder { value: *value, _tcx: PhantomData }.instantiate(self.cx, self.args)
        })
    }
}
```

## Block 37
**Metadata**: AST_ID=37 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=8

```rust
impl<I: Interner, Iter: IntoIterator> ExactSizeIterator for IterInstantiatedCopied<'_, I, Iter>
where
    Iter::IntoIter: ExactSizeIterator,
    Iter::Item: Deref,
    <Iter::Item as Deref>::Target: Copy + TypeFoldable<I>,
{
}
```

## Block 38
**Metadata**: AST_ID=38 | TYPE=STRUCT | NAME=IterIdentityCopied | COMPLEXITY=2 | LINES=4

```rust
pub struct IterIdentityCopied<Iter: IntoIterator> {
    it: Iter::IntoIter,
}
```

## Block 39
**Metadata**: AST_ID=39 | TYPE=FUNCTION | NAME=next | COMPLEXITY=6 | LINES=16

```rust
impl<Iter: IntoIterator> Iterator for IterIdentityCopied<Iter>
where
    Iter::Item: Deref,
    <Iter::Item as Deref>::Target: Copy,
{
    type Item = <Iter::Item as Deref>::Target;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().map(|i| *i)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.it.size_hint()
    }
}
```

## Block 40
**Metadata**: AST_ID=40 | TYPE=FUNCTION | NAME=next_back | COMPLEXITY=5 | LINES=11

```rust
impl<Iter: IntoIterator> DoubleEndedIterator for IterIdentityCopied<Iter>
where
    Iter::IntoIter: DoubleEndedIterator,
    Iter::Item: Deref,
    <Iter::Item as Deref>::Target: Copy,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.it.next_back().map(|i| *i)
    }
}
```

## Block 41
**Metadata**: AST_ID=41 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=8

```rust
impl<Iter: IntoIterator> ExactSizeIterator for IterIdentityCopied<Iter>
where
    Iter::IntoIter: ExactSizeIterator,
    Iter::Item: Deref,
    <Iter::Item as Deref>::Target: Copy,
{
}
```

## Block 42
**Metadata**: AST_ID=42 | TYPE=STRUCT | NAME=EarlyBinderIter | COMPLEXITY=2 | LINES=4

```rust
pub struct EarlyBinderIter<I, T> {
    t: T,
    _tcx: PhantomData<I>,
}
```

## Block 43
**Metadata**: AST_ID=43 | TYPE=FUNCTION | NAME=transpose_iter | COMPLEXITY=4 | LINES=6

```rust
impl<I: Interner, T: IntoIterator> EarlyBinder<I, T> {
    pub fn transpose_iter(self) -> EarlyBinderIter<I, T::IntoIter> {
        EarlyBinderIter { t: self.value.into_iter(), _tcx: PhantomData }
    }
}
```

## Block 44
**Metadata**: AST_ID=44 | TYPE=FUNCTION | NAME=next | COMPLEXITY=7 | LINES=12

```rust
impl<I: Interner, T: Iterator> Iterator for EarlyBinderIter<I, T> {
    type Item = EarlyBinder<I, T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        self.t.next().map(|value| EarlyBinder { value, _tcx: PhantomData })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.t.size_hint()
    }
}
```

## Block 45
**Metadata**: AST_ID=45 | TYPE=FUNCTION | NAME=instantiate | COMPLEXITY=20 | LINES=38

```rust
impl<I: Interner, T: TypeFoldable<I>> ty::EarlyBinder<I, T> {
    pub fn instantiate<A>(self, cx: I, args: A) -> T
    where
        A: SliceLike<Item = I::GenericArg>,
    {
        // Nothing to fold, so let's avoid visiting things and possibly re-hashing/equating
        // them when interning. Perf testing found this to be a modest improvement.
        // See: <https://github.com/rust-lang/rust/pull/142317>
        if args.is_empty() {
            assert!(
                !self.value.has_param(),
                "{:?} has parameters, but no args were provided in instantiate",
                self.value,
            );
            return self.value;
        }
        let mut folder = ArgFolder { cx, args: args.as_slice(), binders_passed: 0 };
        self.value.fold_with(&mut folder)
    }

    /// Makes the identity replacement `T0 => T0, ..., TN => TN`.
    /// Conceptually, this converts universally bound variables into placeholders
    /// when inside of a given item.
    ///
    /// For example, consider `for<T> fn foo<T>(){ .. }`:
    /// - Outside of `foo`, `T` is bound (represented by the presence of `EarlyBinder`).
    /// - Inside of the body of `foo`, we treat `T` as a placeholder by calling
    /// `instantiate_identity` to discharge the `EarlyBinder`.
    pub fn instantiate_identity(self) -> T {
        self.value
    }

    /// Returns the inner value, but only if it contains no bound vars.
    pub fn no_bound_vars(self) -> Option<T> {
        if !self.value.has_param() { Some(self.value) } else { None }
    }
}
```

## Block 46
**Metadata**: AST_ID=46 | TYPE=STRUCT | NAME=ArgFolder | COMPLEXITY=4 | LINES=11

```rust
///////////////////////////////////////////////////////////////////////////
// The actual instantiation engine itself is a type folder.

struct ArgFolder<'a, I: Interner> {
    cx: I,
    args: &'a [I::GenericArg],

    /// Number of region binders we have passed through while doing the instantiation
    binders_passed: u32,
}
```

## Block 47
**Metadata**: AST_ID=47 | TYPE=FUNCTION | NAME=cx | COMPLEXITY=44 | LINES=66

```rust
impl<'a, I: Interner> TypeFolder<I> for ArgFolder<'a, I> {
    #[inline]
    fn cx(&self) -> I {
        self.cx
    }

    fn fold_binder<T: TypeFoldable<I>>(&mut self, t: ty::Binder<I, T>) -> ty::Binder<I, T> {
        self.binders_passed += 1;
        let t = t.super_fold_with(self);
        self.binders_passed -= 1;
        t
    }

    fn fold_region(&mut self, r: I::Region) -> I::Region {
        // Note: This routine only handles regions that are bound on
        // type declarations and other outer declarations, not those
        // bound in *fn types*. Region instantiation of the bound
        // regions that appear in a function signature is done using
        // the specialized routine `ty::replace_late_regions()`.
        match r.kind() {
            ty::ReEarlyParam(data) => {
                let rk = self.args.get(data.index() as usize).map(|arg| arg.kind());
                match rk {
                    Some(ty::GenericArgKind::Lifetime(lt)) => self.shift_region_through_binders(lt),
                    Some(other) => self.region_param_expected(data, r, other),
                    None => self.region_param_out_of_range(data, r),
                }
            }
            ty::ReBound(..)
            | ty::ReLateParam(_)
            | ty::ReStatic
            | ty::RePlaceholder(_)
            | ty::ReErased
            | ty::ReError(_) => r,
            ty::ReVar(_) => panic!("unexpected region: {r:?}"),
        }
    }

    fn fold_ty(&mut self, t: I::Ty) -> I::Ty {
        if !t.has_param() {
            return t;
        }

        match t.kind() {
            ty::Param(p) => self.ty_for_param(p, t),
            _ => t.super_fold_with(self),
        }
    }

    fn fold_const(&mut self, c: I::Const) -> I::Const {
        if let ty::ConstKind::Param(p) = c.kind() {
            self.const_for_param(p, c)
        } else {
            c.super_fold_with(self)
        }
    }

    fn fold_predicate(&mut self, p: I::Predicate) -> I::Predicate {
        if p.has_param() { p.super_fold_with(self) } else { p }
    }

    fn fold_clauses(&mut self, c: I::Clauses) -> I::Clauses {
        if c.has_param() { c.super_fold_with(self) } else { c }
    }
}
```

## Block 48
**Metadata**: AST_ID=48 | TYPE=FUNCTION | NAME=ty_for_param | COMPLEXITY=74 | LINES=170

```rust
impl<'a, I: Interner> ArgFolder<'a, I> {
    fn ty_for_param(&self, p: I::ParamTy, source_ty: I::Ty) -> I::Ty {
        // Look up the type in the args. It really should be in there.
        let opt_ty = self.args.get(p.index() as usize).map(|arg| arg.kind());
        let ty = match opt_ty {
            Some(ty::GenericArgKind::Type(ty)) => ty,
            Some(kind) => self.type_param_expected(p, source_ty, kind),
            None => self.type_param_out_of_range(p, source_ty),
        };

        self.shift_vars_through_binders(ty)
    }

    #[cold]
    #[inline(never)]
    fn type_param_expected(&self, p: I::ParamTy, ty: I::Ty, kind: ty::GenericArgKind<I>) -> ! {
        panic!(
            "expected type for `{:?}` ({:?}/{}) but found {:?} when instantiating, args={:?}",
            p,
            ty,
            p.index(),
            kind,
            self.args,
        )
    }

    #[cold]
    #[inline(never)]
    fn type_param_out_of_range(&self, p: I::ParamTy, ty: I::Ty) -> ! {
        panic!(
            "type parameter `{:?}` ({:?}/{}) out of range when instantiating, args={:?}",
            p,
            ty,
            p.index(),
            self.args,
        )
    }

    fn const_for_param(&self, p: I::ParamConst, source_ct: I::Const) -> I::Const {
        // Look up the const in the args. It really should be in there.
        let opt_ct = self.args.get(p.index() as usize).map(|arg| arg.kind());
        let ct = match opt_ct {
            Some(ty::GenericArgKind::Const(ct)) => ct,
            Some(kind) => self.const_param_expected(p, source_ct, kind),
            None => self.const_param_out_of_range(p, source_ct),
        };

        self.shift_vars_through_binders(ct)
    }

    #[cold]
    #[inline(never)]
    fn const_param_expected(
        &self,
        p: I::ParamConst,
        ct: I::Const,
        kind: ty::GenericArgKind<I>,
    ) -> ! {
        panic!(
            "expected const for `{:?}` ({:?}/{}) but found {:?} when instantiating args={:?}",
            p,
            ct,
            p.index(),
            kind,
            self.args,
        )
    }

    #[cold]
    #[inline(never)]
    fn const_param_out_of_range(&self, p: I::ParamConst, ct: I::Const) -> ! {
        panic!(
            "const parameter `{:?}` ({:?}/{}) out of range when instantiating args={:?}",
            p,
            ct,
            p.index(),
            self.args,
        )
    }

    #[cold]
    #[inline(never)]
    fn region_param_expected(
        &self,
        ebr: I::EarlyParamRegion,
        r: I::Region,
        kind: ty::GenericArgKind<I>,
    ) -> ! {
        panic!(
            "expected region for `{:?}` ({:?}/{}) but found {:?} when instantiating args={:?}",
            ebr,
            r,
            ebr.index(),
            kind,
            self.args,
        )
    }

    #[cold]
    #[inline(never)]
    fn region_param_out_of_range(&self, ebr: I::EarlyParamRegion, r: I::Region) -> ! {
        panic!(
            "region parameter `{:?}` ({:?}/{}) out of range when instantiating args={:?}",
            ebr,
            r,
            ebr.index(),
            self.args,
        )
    }

    /// It is sometimes necessary to adjust the De Bruijn indices during instantiation. This occurs
    /// when we are instantiating a type with escaping bound vars into a context where we have
    /// passed through binders. That's quite a mouthful. Let's see an example:
    ///
    /// ```
    /// type Func<A> = fn(A);
    /// type MetaFunc = for<'a> fn(Func<&'a i32>);
    /// ```
    ///
    /// The type `MetaFunc`, when fully expanded, will be
    /// ```ignore (illustrative)
    /// for<'a> fn(fn(&'a i32))
    /// //      ^~ ^~ ^~~
    /// //      |  |  |
    /// //      |  |  DebruijnIndex of 2
    /// //      Binders
    /// ```
    /// Here the `'a` lifetime is bound in the outer function, but appears as an argument of the
    /// inner one. Therefore, that appearance will have a DebruijnIndex of 2, because we must skip
    /// over the inner binder (remember that we count De Bruijn indices from 1). However, in the
    /// definition of `MetaFunc`, the binder is not visible, so the type `&'a i32` will have a
    /// De Bruijn index of 1. It's only during the instantiation that we can see we must increase the
    /// depth by 1 to account for the binder that we passed through.
    ///
    /// As a second example, consider this twist:
    ///
    /// ```
    /// type FuncTuple<A> = (A,fn(A));
    /// type MetaFuncTuple = for<'a> fn(FuncTuple<&'a i32>);
    /// ```
    ///
    /// Here the final type will be:
    /// ```ignore (illustrative)
    /// for<'a> fn((&'a i32, fn(&'a i32)))
    /// //          ^~~         ^~~
    /// //          |           |
    /// //   DebruijnIndex of 1 |
    /// //               DebruijnIndex of 2
    /// ```
    /// As indicated in the diagram, here the same type `&'a i32` is instantiated once, but in the
    /// first case we do not increase the De Bruijn index and in the second case we do. The reason
    /// is that only in the second case have we passed through a fn binder.
    #[instrument(level = "trace", skip(self), fields(binders_passed = self.binders_passed), ret)]
    fn shift_vars_through_binders<T: TypeFoldable<I>>(&self, val: T) -> T {
        if self.binders_passed == 0 || !val.has_escaping_bound_vars() {
            val
        } else {
            ty::shift_vars(self.cx, val, self.binders_passed)
        }
    }

    fn shift_region_through_binders(&self, region: I::Region) -> I::Region {
        if self.binders_passed == 0 || !region.has_escaping_bound_vars() {
            region
        } else {
            ty::shift_region(self.cx, region, self.binders_passed)
        }
    }
}
```

---
*Generated by AST tracing system*
