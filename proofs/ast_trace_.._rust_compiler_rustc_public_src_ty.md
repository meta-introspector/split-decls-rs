# AST Trace: ../rust/compiler/rustc_public/src/ty.rs

Generated 139 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use std::fmt::{self, Debug, Display, Formatter};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
use std::ops::Range;

use serde::Serialize;

use super::abi::ReprOptions;
use super::mir::{Body, Mutability, Safety};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use super::{DefId, Error, Symbol, with};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::abi::{FnAbi, Layout};
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::crate_def::{CrateDef, CrateDefItems, CrateDefType};
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::mir::alloc::{AllocId, read_target_int, read_target_uint};
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use crate::mir::mono::StaticDef;
use crate::target::MachineInfo;
use crate::{Filename, IndexedVal, Opaque};
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=Ty(usize); | COMPLEXITY=5 | LINES=9

```rust
#[derive(Copy, Clone, Eq, PartialEq, Hash, Serialize)]
pub struct Ty(usize);

impl Debug for Ty {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ty").field("id", &self.0).field("kind", &self.kind()).finish()
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=from_rigid_kind | COMPLEXITY=25 | LINES=78

```rust
/// Constructors for `Ty`.
impl Ty {
    /// Create a new type from a given kind.
    pub fn from_rigid_kind(kind: RigidTy) -> Ty {
        with(|cx| cx.new_rigid_ty(kind))
    }

    /// Create a new array type.
    pub fn try_new_array(elem_ty: Ty, size: u64) -> Result<Ty, Error> {
        Ok(Ty::from_rigid_kind(RigidTy::Array(elem_ty, TyConst::try_from_target_usize(size)?)))
    }

    /// Create a new array type from Const length.
    pub fn new_array_with_const_len(elem_ty: Ty, len: TyConst) -> Ty {
        Ty::from_rigid_kind(RigidTy::Array(elem_ty, len))
    }

    /// Create a new pointer type.
    pub fn new_ptr(pointee_ty: Ty, mutability: Mutability) -> Ty {
        Ty::from_rigid_kind(RigidTy::RawPtr(pointee_ty, mutability))
    }

    /// Create a new reference type.
    pub fn new_ref(reg: Region, pointee_ty: Ty, mutability: Mutability) -> Ty {
        Ty::from_rigid_kind(RigidTy::Ref(reg, pointee_ty, mutability))
    }

    /// Create a new pointer type.
    pub fn new_tuple(tys: &[Ty]) -> Ty {
        Ty::from_rigid_kind(RigidTy::Tuple(Vec::from(tys)))
    }

    /// Create a new closure type.
    pub fn new_closure(def: ClosureDef, args: GenericArgs) -> Ty {
        Ty::from_rigid_kind(RigidTy::Closure(def, args))
    }

    /// Create a new coroutine type.
    pub fn new_coroutine(def: CoroutineDef, args: GenericArgs) -> Ty {
        Ty::from_rigid_kind(RigidTy::Coroutine(def, args))
    }

    /// Create a new closure type.
    pub fn new_coroutine_closure(def: CoroutineClosureDef, args: GenericArgs) -> Ty {
        Ty::from_rigid_kind(RigidTy::CoroutineClosure(def, args))
    }

    /// Create a new box type that represents `Box<T>`, for the given inner type `T`.
    pub fn new_box(inner_ty: Ty) -> Ty {
        with(|cx| cx.new_box_ty(inner_ty))
    }

    /// Create a type representing `usize`.
    pub fn usize_ty() -> Ty {
        Ty::from_rigid_kind(RigidTy::Uint(UintTy::Usize))
    }

    /// Create a type representing `bool`.
    pub fn bool_ty() -> Ty {
        Ty::from_rigid_kind(RigidTy::Bool)
    }

    /// Create a type representing a signed integer.
    pub fn signed_ty(inner: IntTy) -> Ty {
        Ty::from_rigid_kind(RigidTy::Int(inner))
    }

    /// Create a type representing an unsigned integer.
    pub fn unsigned_ty(inner: UintTy) -> Ty {
        Ty::from_rigid_kind(RigidTy::Uint(inner))
    }

    /// Get a type layout.
    pub fn layout(self) -> Result<Layout, Error> {
        with(|cx| cx.ty_layout(self))
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=kind | COMPLEXITY=3 | LINES=6

```rust
impl Ty {
    pub fn kind(&self) -> TyKind {
        with(|context| context.ty_kind(*self))
    }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=3 | LINES=6

```rust
/// Represents a pattern in the type system
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum Pattern {
    Range { start: Option<TyConst>, end: Option<TyConst>, include_end: bool },
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=STRUCT | NAME=TyConst | COMPLEXITY=2 | LINES=7

```rust
/// Represents a constant in the type system
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub struct TyConst {
    pub(crate) kind: TyConstKind,
    pub id: TyConstId,
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=new | COMPLEXITY=8 | LINES=21

```rust
impl TyConst {
    pub fn new(kind: TyConstKind, id: TyConstId) -> TyConst {
        Self { kind, id }
    }

    /// Retrieve the constant kind.
    pub fn kind(&self) -> &TyConstKind {
        &self.kind
    }

    /// Creates an interned usize constant.
    pub fn try_from_target_usize(val: u64) -> Result<Self, Error> {
        with(|cx| cx.try_new_ty_const_uint(val.into(), UintTy::Usize))
    }

    /// Try to evaluate to a target `usize`.
    pub fn eval_target_usize(&self) -> Result<u64, Error> {
        with(|cx| cx.eval_target_usize_ty(self))
    }
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=11

```rust
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub enum TyConstKind {
    Param(ParamConst),
    Bound(DebruijnIndex, BoundVar),
    Unevaluated(ConstDef, GenericArgs),

    // FIXME: These should be a valtree
    Value(Ty, Allocation),
    ZSTValue(Ty),
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=STRUCT | NAME=TyConstId(usize); | COMPLEXITY=4 | LINES=14

```rust
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub struct TyConstId(usize);

/// Represents a constant in MIR
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub struct MirConst {
    /// The constant kind.
    pub(crate) kind: ConstantKind,
    /// The constant type.
    pub(crate) ty: Ty,
    /// Used for internal tracking of the internal constant.
    pub id: MirConstId,
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=new | COMPLEXITY=18 | LINES=47

```rust
impl MirConst {
    /// Build a constant. Note that this should only be used by the compiler.
    pub fn new(kind: ConstantKind, ty: Ty, id: MirConstId) -> MirConst {
        MirConst { kind, ty, id }
    }

    /// Retrieve the constant kind.
    pub fn kind(&self) -> &ConstantKind {
        &self.kind
    }

    /// Get the constant type.
    pub fn ty(&self) -> Ty {
        self.ty
    }

    /// Try to evaluate to a target `usize`.
    pub fn eval_target_usize(&self) -> Result<u64, Error> {
        with(|cx| cx.eval_target_usize(self))
    }

    /// Create a constant that represents a new zero-sized constant of type T.
    /// Fails if the type is not a ZST or if it doesn't have a known size.
    pub fn try_new_zero_sized(ty: Ty) -> Result<MirConst, Error> {
        with(|cx| cx.try_new_const_zst(ty))
    }

    /// Build a new constant that represents the given string.
    ///
    /// Note that there is no guarantee today about duplication of the same constant.
    /// I.e.: Calling this function multiple times with the same argument may or may not return
    /// the same allocation.
    pub fn from_str(value: &str) -> MirConst {
        with(|cx| cx.new_const_str(value))
    }

    /// Build a new constant that represents the given boolean value.
    pub fn from_bool(value: bool) -> MirConst {
        with(|cx| cx.new_const_bool(value))
    }

    /// Build a new constant that represents the given unsigned integer.
    pub fn try_from_uint(value: u128, uint_ty: UintTy) -> Result<MirConst, Error> {
        with(|cx| cx.try_new_const_uint(value, uint_ty))
    }
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=STRUCT | NAME=MirConstId(usize); | COMPLEXITY=2 | LINES=10

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct MirConstId(usize);

type Ident = Opaque;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub struct Region {
    pub kind: RegionKind,
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub enum RegionKind {
    ReEarlyParam(EarlyParamRegion),
    ReBound(DebruijnIndex, BoundRegion),
    ReStatic,
    RePlaceholder(Placeholder<BoundRegion>),
    ReErased,
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=STRUCT | NAME=EarlyParamRegion | COMPLEXITY=2 | LINES=8

```rust
pub(crate) type DebruijnIndex = u32;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub struct EarlyParamRegion {
    pub index: u32,
    pub name: Symbol,
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=STRUCT | NAME=BoundRegion | COMPLEXITY=2 | LINES=8

```rust
pub(crate) type BoundVar = u32;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub struct BoundRegion {
    pub var: BoundVar,
    pub kind: BoundRegionKind,
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=STRUCT | NAME=Placeholder | COMPLEXITY=2 | LINES=8

```rust
pub(crate) type UniverseIndex = u32;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub struct Placeholder<T> {
    pub universe: UniverseIndex,
    pub bound: T,
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=Span(usize); | COMPLEXITY=5 | LINES=12

```rust
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct Span(usize);

impl Debug for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Span")
            .field("id", &self.0)
            .field("repr", &with(|cx| cx.span_to_string(*self)))
            .finish()
    }
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=FUNCTION | NAME=get_filename | COMPLEXITY=8 | LINES=20

```rust
impl Span {
    /// Return filename for diagnostic purposes
    pub fn get_filename(&self) -> Filename {
        with(|c| c.get_filename(self))
    }

    /// Return lines that correspond to this `Span`
    pub fn get_lines(&self) -> LineInfo {
        with(|c| c.get_lines(self))
    }

    /// Return the span location to be printed in diagnostic messages.
    ///
    /// This may leak local file paths and should not be used to build artifacts that may be
    /// distributed.
    pub fn diagnostic(&self) -> String {
        with(|c| c.span_to_string(*self))
    }
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=STRUCT | NAME=LineInfo | COMPLEXITY=2 | LINES=10

```rust
#[derive(Clone, Copy, Debug, Serialize)]
/// Information you get from `Span` in a struct form.
/// Line and col start from 1.
pub struct LineInfo {
    pub start_line: usize,
    pub start_col: usize,
    pub end_line: usize,
    pub end_col: usize,
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=FUNCTION | NAME=from | COMPLEXITY=4 | LINES=6

```rust
impl LineInfo {
    pub fn from(lines: (usize, usize, usize, usize)) -> Self {
        LineInfo { start_line: lines.0, start_col: lines.1, end_line: lines.2, end_col: lines.3 }
    }
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum TyKind {
    RigidTy(RigidTy),
    Alias(AliasKind, AliasTy),
    Param(ParamTy),
    Bound(usize, BoundTy),
}
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=FUNCTION | NAME=rigid | COMPLEXITY=118 | LINES=226

```rust
impl TyKind {
    pub fn rigid(&self) -> Option<&RigidTy> {
        if let TyKind::RigidTy(inner) = self { Some(inner) } else { None }
    }

    #[inline]
    pub fn is_unit(&self) -> bool {
        matches!(self, TyKind::RigidTy(RigidTy::Tuple(data)) if data.is_empty())
    }

    #[inline]
    pub fn is_bool(&self) -> bool {
        matches!(self, TyKind::RigidTy(RigidTy::Bool))
    }

    #[inline]
    pub fn is_char(&self) -> bool {
        matches!(self, TyKind::RigidTy(RigidTy::Char))
    }

    #[inline]
    pub fn is_trait(&self) -> bool {
        matches!(self, TyKind::RigidTy(RigidTy::Dynamic(_, _, DynKind::Dyn)))
    }

    #[inline]
    pub fn is_enum(&self) -> bool {
        matches!(self, TyKind::RigidTy(RigidTy::Adt(def, _)) if def.kind() == AdtKind::Enum)
    }

    #[inline]
    pub fn is_struct(&self) -> bool {
        matches!(self, TyKind::RigidTy(RigidTy::Adt(def, _)) if def.kind() == AdtKind::Struct)
    }

    #[inline]
    pub fn is_union(&self) -> bool {
        matches!(self, TyKind::RigidTy(RigidTy::Adt(def, _)) if def.kind() == AdtKind::Union)
    }

    #[inline]
    pub fn is_adt(&self) -> bool {
        matches!(self, TyKind::RigidTy(RigidTy::Adt(..)))
    }

    #[inline]
    pub fn is_ref(&self) -> bool {
        matches!(self, TyKind::RigidTy(RigidTy::Ref(..)))
    }

    #[inline]
    pub fn is_fn(&self) -> bool {
        matches!(self, TyKind::RigidTy(RigidTy::FnDef(..)))
    }

    #[inline]
    pub fn is_fn_ptr(&self) -> bool {
        matches!(self, TyKind::RigidTy(RigidTy::FnPtr(..)))
    }

    #[inline]
    pub fn is_primitive(&self) -> bool {
        matches!(
            self,
            TyKind::RigidTy(
                RigidTy::Bool
                    | RigidTy::Char
                    | RigidTy::Int(_)
                    | RigidTy::Uint(_)
                    | RigidTy::Float(_)
            )
        )
    }

    #[inline]
    pub fn is_float(&self) -> bool {
        matches!(self, TyKind::RigidTy(RigidTy::Float(_)))
    }

    #[inline]
    pub fn is_integral(&self) -> bool {
        matches!(self, TyKind::RigidTy(RigidTy::Int(_) | RigidTy::Uint(_)))
    }

    #[inline]
    pub fn is_numeric(&self) -> bool {
        self.is_integral() || self.is_float()
    }

    #[inline]
    pub fn is_signed(&self) -> bool {
        matches!(self, TyKind::RigidTy(RigidTy::Int(_)))
    }

    #[inline]
    pub fn is_str(&self) -> bool {
        *self == TyKind::RigidTy(RigidTy::Str)
    }

    #[inline]
    pub fn is_cstr(&self) -> bool {
        let TyKind::RigidTy(RigidTy::Adt(def, _)) = self else {
            return false;
        };
        with(|cx| cx.adt_is_cstr(*def))
    }

    #[inline]
    pub fn is_slice(&self) -> bool {
        matches!(self, TyKind::RigidTy(RigidTy::Slice(_)))
    }

    #[inline]
    pub fn is_array(&self) -> bool {
        matches!(self, TyKind::RigidTy(RigidTy::Array(..)))
    }

    #[inline]
    pub fn is_mutable_ptr(&self) -> bool {
        matches!(
            self,
            TyKind::RigidTy(RigidTy::RawPtr(_, Mutability::Mut))
                | TyKind::RigidTy(RigidTy::Ref(_, _, Mutability::Mut))
        )
    }

    #[inline]
    pub fn is_raw_ptr(&self) -> bool {
        matches!(self, TyKind::RigidTy(RigidTy::RawPtr(..)))
    }

    /// Tests if this is any kind of primitive pointer type (reference, raw pointer, fn pointer).
    #[inline]
    pub fn is_any_ptr(&self) -> bool {
        self.is_ref() || self.is_raw_ptr() || self.is_fn_ptr()
    }

    #[inline]
    pub fn is_coroutine(&self) -> bool {
        matches!(self, TyKind::RigidTy(RigidTy::Coroutine(..)))
    }

    #[inline]
    pub fn is_closure(&self) -> bool {
        matches!(self, TyKind::RigidTy(RigidTy::Closure(..)))
    }

    #[inline]
    pub fn is_box(&self) -> bool {
        match self {
            TyKind::RigidTy(RigidTy::Adt(def, _)) => def.is_box(),
            _ => false,
        }
    }

    #[inline]
    pub fn is_simd(&self) -> bool {
        matches!(self, TyKind::RigidTy(RigidTy::Adt(def, _)) if def.is_simd())
    }

    pub fn trait_principal(&self) -> Option<Binder<ExistentialTraitRef>> {
        if let TyKind::RigidTy(RigidTy::Dynamic(predicates, _, _)) = self {
            if let Some(Binder { value: ExistentialPredicate::Trait(trait_ref), bound_vars }) =
                predicates.first()
            {
                Some(Binder { value: trait_ref.clone(), bound_vars: bound_vars.clone() })
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Returns the type of `ty[i]` for builtin types.
    pub fn builtin_index(&self) -> Option<Ty> {
        match self.rigid()? {
            RigidTy::Array(ty, _) | RigidTy::Slice(ty) => Some(*ty),
            _ => None,
        }
    }

    /// Returns the type and mutability of `*ty` for builtin types.
    ///
    /// The parameter `explicit` indicates if this is an *explicit* dereference.
    /// Some types -- notably raw ptrs -- can only be dereferenced explicitly.
    pub fn builtin_deref(&self, explicit: bool) -> Option<TypeAndMut> {
        match self.rigid()? {
            RigidTy::Adt(def, args) if def.is_box() => {
                Some(TypeAndMut { ty: *args.0.first()?.ty()?, mutability: Mutability::Not })
            }
            RigidTy::Ref(_, ty, mutability) => {
                Some(TypeAndMut { ty: *ty, mutability: *mutability })
            }
            RigidTy::RawPtr(ty, mutability) if explicit => {
                Some(TypeAndMut { ty: *ty, mutability: *mutability })
            }
            _ => None,
        }
    }

    /// Get the function signature for function like types (Fn, FnPtr, and Closure)
    pub fn fn_sig(&self) -> Option<PolyFnSig> {
        match self {
            TyKind::RigidTy(RigidTy::FnDef(def, args)) => Some(with(|cx| cx.fn_sig(*def, args))),
            TyKind::RigidTy(RigidTy::FnPtr(sig)) => Some(sig.clone()),
            TyKind::RigidTy(RigidTy::Closure(_def, args)) => Some(with(|cx| cx.closure_sig(args))),
            _ => None,
        }
    }

    /// Get the discriminant type for this type.
    pub fn discriminant_ty(&self) -> Option<Ty> {
        self.rigid().map(|ty| with(|cx| cx.rigid_ty_discriminant_ty(ty)))
    }

    /// Deconstruct a function type if this is one.
    pub fn fn_def(&self) -> Option<(FnDef, &GenericArgs)> {
        if let TyKind::RigidTy(RigidTy::FnDef(def, args)) = self {
            Some((*def, args))
        } else {
            None
        }
    }
}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=STRUCT | NAME=TypeAndMut | COMPLEXITY=2 | LINES=5

```rust
pub struct TypeAndMut {
    pub ty: Ty,
    pub mutability: Mutability,
}
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=3 | LINES=26

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum RigidTy {
    Bool,
    Char,
    Int(IntTy),
    Uint(UintTy),
    Float(FloatTy),
    Adt(AdtDef, GenericArgs),
    Foreign(ForeignDef),
    Str,
    Array(Ty, TyConst),
    Pat(Ty, Pattern),
    Slice(Ty),
    RawPtr(Ty, Mutability),
    Ref(Region, Ty, Mutability),
    FnDef(FnDef, GenericArgs),
    FnPtr(PolyFnSig),
    Closure(ClosureDef, GenericArgs),
    Coroutine(CoroutineDef, GenericArgs),
    CoroutineClosure(CoroutineClosureDef, GenericArgs),
    Dynamic(Vec<Binder<ExistentialPredicate>>, Region, DynKind),
    Never,
    Tuple(Vec<Ty>),
    CoroutineWitness(CoroutineWitnessDef, GenericArgs),
}
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=FUNCTION | NAME=discriminant_ty | COMPLEXITY=5 | LINES=7

```rust
impl RigidTy {
    /// Get the discriminant type for this type.
    pub fn discriminant_ty(&self) -> Ty {
        with(|cx| cx.rigid_ty_discriminant_ty(self))
    }
}
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=6

```rust
impl From<RigidTy> for TyKind {
    fn from(value: RigidTy) -> Self {
        TyKind::RigidTy(value)
    }
}
```

## Block 32
**Metadata**: AST_ID=32 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum IntTy {
    Isize,
    I8,
    I16,
    I32,
    I64,
    I128,
}
```

## Block 33
**Metadata**: AST_ID=33 | TYPE=FUNCTION | NAME=num_bytes | COMPLEXITY=7 | LINES=13

```rust
impl IntTy {
    pub fn num_bytes(self) -> usize {
        match self {
            IntTy::Isize => MachineInfo::target_pointer_width().bytes(),
            IntTy::I8 => 1,
            IntTy::I16 => 2,
            IntTy::I32 => 4,
            IntTy::I64 => 8,
            IntTy::I128 => 16,
        }
    }
}
```

## Block 34
**Metadata**: AST_ID=34 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum UintTy {
    Usize,
    U8,
    U16,
    U32,
    U64,
    U128,
}
```

## Block 35
**Metadata**: AST_ID=35 | TYPE=FUNCTION | NAME=num_bytes | COMPLEXITY=7 | LINES=13

```rust
impl UintTy {
    pub fn num_bytes(self) -> usize {
        match self {
            UintTy::Usize => MachineInfo::target_pointer_width().bytes(),
            UintTy::U8 => 1,
            UintTy::U16 => 2,
            UintTy::U32 => 4,
            UintTy::U64 => 8,
            UintTy::U128 => 16,
        }
    }
}
```

## Block 36
**Metadata**: AST_ID=36 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum FloatTy {
    F16,
    F32,
    F64,
    F128,
}
```

## Block 37
**Metadata**: AST_ID=37 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Movability {
    Static,
    Movable,
}
```

## Block 38
**Metadata**: AST_ID=38 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
crate_def! {
    #[derive(Serialize)]
    pub ForeignModuleDef;
}
```

## Block 39
**Metadata**: AST_ID=39 | TYPE=FUNCTION | NAME=module | COMPLEXITY=3 | LINES=6

```rust
impl ForeignModuleDef {
    pub fn module(&self) -> ForeignModule {
        with(|cx| cx.foreign_module(*self))
    }
}
```

## Block 40
**Metadata**: AST_ID=40 | TYPE=STRUCT | NAME=ForeignModule | COMPLEXITY=2 | LINES=5

```rust
pub struct ForeignModule {
    pub def_id: ForeignModuleDef,
    pub abi: Abi,
}
```

## Block 41
**Metadata**: AST_ID=41 | TYPE=FUNCTION | NAME=items | COMPLEXITY=3 | LINES=6

```rust
impl ForeignModule {
    pub fn items(&self) -> Vec<ForeignDef> {
        with(|cx| cx.foreign_items(self.def_id))
    }
}
```

## Block 42
**Metadata**: AST_ID=42 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
crate_def_with_ty! {
    /// Hold information about a ForeignItem in a crate.
    #[derive(Serialize)]
    pub ForeignDef;
}
```

## Block 43
**Metadata**: AST_ID=43 | TYPE=FUNCTION | NAME=kind | COMPLEXITY=3 | LINES=6

```rust
impl ForeignDef {
    pub fn kind(&self) -> ForeignItemKind {
        with(|cx| cx.foreign_item_kind(*self))
    }
}
```

## Block 44
**Metadata**: AST_ID=44 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Serialize)]
pub enum ForeignItemKind {
    Fn(FnDef),
    Static(StaticDef),
    Type(Ty),
}
```

## Block 45
**Metadata**: AST_ID=45 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
crate_def_with_ty! {
    /// Hold information about a function definition in a crate.
    #[derive(Serialize)]
    pub FnDef;
}
```

## Block 46
**Metadata**: AST_ID=46 | TYPE=FUNCTION | NAME=body | COMPLEXITY=18 | LINES=29

```rust
impl FnDef {
    // Get the function body if available.
    pub fn body(&self) -> Option<Body> {
        with(|ctx| ctx.has_body(self.0).then(|| ctx.mir_body(self.0)))
    }

    // Check if the function body is available.
    pub fn has_body(&self) -> bool {
        with(|ctx| ctx.has_body(self.0))
    }

    /// Get the information of the intrinsic if this function is a definition of one.
    pub fn as_intrinsic(&self) -> Option<IntrinsicDef> {
        with(|cx| cx.intrinsic(self.def_id()))
    }

    /// Check if the function is an intrinsic.
    #[inline]
    pub fn is_intrinsic(&self) -> bool {
        self.as_intrinsic().is_some()
    }

    /// Get the function signature for this function definition.
    pub fn fn_sig(&self) -> PolyFnSig {
        let kind = self.ty().kind();
        kind.fn_sig().unwrap()
    }
}
```

## Block 47
**Metadata**: AST_ID=47 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
crate_def_with_ty! {
    #[derive(Serialize)]
    pub IntrinsicDef;
}
```

## Block 48
**Metadata**: AST_ID=48 | TYPE=FUNCTION | NAME=fn_name | COMPLEXITY=6 | LINES=14

```rust
impl IntrinsicDef {
    /// Returns the plain name of the intrinsic.
    /// e.g., `transmute` for `core::intrinsics::transmute`.
    pub fn fn_name(&self) -> Symbol {
        with(|cx| cx.intrinsic_name(*self))
    }

    /// Returns whether the intrinsic has no meaningful body and all backends
    /// need to shim all calls to it.
    pub fn must_be_overridden(&self) -> bool {
        with(|cx| !cx.has_body(self.0))
    }
}
```

## Block 49
**Metadata**: AST_ID=49 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=6

```rust
impl From<IntrinsicDef> for FnDef {
    fn from(def: IntrinsicDef) -> Self {
        FnDef(def.0)
    }
}
```

## Block 50
**Metadata**: AST_ID=50 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
crate_def! {
    #[derive(Serialize)]
    pub ClosureDef;
}
```

## Block 51
**Metadata**: AST_ID=51 | TYPE=FUNCTION | NAME=body | COMPLEXITY=5 | LINES=8

```rust
impl ClosureDef {
    /// Retrieves the body of the closure definition. Returns None if the body
    /// isn't available.
    pub fn body(&self) -> Option<Body> {
        with(|ctx| ctx.has_body(self.0).then(|| ctx.mir_body(self.0)))
    }
}
```

## Block 52
**Metadata**: AST_ID=52 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
crate_def! {
    #[derive(Serialize)]
    pub CoroutineDef;
}
```

## Block 53
**Metadata**: AST_ID=53 | TYPE=FUNCTION | NAME=body | COMPLEXITY=6 | LINES=12

```rust
impl CoroutineDef {
    /// Retrieves the body of the coroutine definition. Returns None if the body
    /// isn't available.
    pub fn body(&self) -> Option<Body> {
        with(|cx| cx.has_body(self.0).then(|| cx.mir_body(self.0)))
    }

    pub fn discriminant_for_variant(&self, args: &GenericArgs, idx: VariantIdx) -> Discr {
        with(|cx| cx.coroutine_discr_for_variant(*self, args, idx))
    }
}
```

## Block 54
**Metadata**: AST_ID=54 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
crate_def! {
    #[derive(Serialize)]
    pub CoroutineClosureDef;
}
```

## Block 55
**Metadata**: AST_ID=55 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
crate_def! {
    #[derive(Serialize)]
    pub ParamDef;
}
```

## Block 56
**Metadata**: AST_ID=56 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
crate_def! {
    #[derive(Serialize)]
    pub BrNamedDef;
}
```

## Block 57
**Metadata**: AST_ID=57 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
crate_def! {
    #[derive(Serialize)]
    pub AdtDef;
}
```

## Block 58
**Metadata**: AST_ID=58 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Serialize)]
pub enum AdtKind {
    Enum,
    Union,
    Struct,
}
```

## Block 59
**Metadata**: AST_ID=59 | TYPE=FUNCTION | NAME=kind | COMPLEXITY=18 | LINES=54

```rust
impl AdtDef {
    pub fn kind(&self) -> AdtKind {
        with(|cx| cx.adt_kind(*self))
    }

    /// Retrieve the type of this Adt.
    pub fn ty(&self) -> Ty {
        with(|cx| cx.def_ty(self.0))
    }

    /// Retrieve the type of this Adt by instantiating and normalizing it with the given arguments.
    ///
    /// This will assume the type can be instantiated with these arguments.
    pub fn ty_with_args(&self, args: &GenericArgs) -> Ty {
        with(|cx| cx.def_ty_with_args(self.0, args))
    }

    pub fn is_box(&self) -> bool {
        with(|cx| cx.adt_is_box(*self))
    }

    pub fn is_simd(&self) -> bool {
        with(|cx| cx.adt_is_simd(*self))
    }

    /// The number of variants in this ADT.
    pub fn num_variants(&self) -> usize {
        with(|cx| cx.adt_variants_len(*self))
    }

    /// Retrieve the variants in this ADT.
    pub fn variants(&self) -> Vec<VariantDef> {
        self.variants_iter().collect()
    }

    /// Iterate over the variants in this ADT.
    pub fn variants_iter(&self) -> impl Iterator<Item = VariantDef> {
        (0..self.num_variants())
            .map(|idx| VariantDef { idx: VariantIdx::to_val(idx), adt_def: *self })
    }

    pub fn variant(&self, idx: VariantIdx) -> Option<VariantDef> {
        (idx.to_index() < self.num_variants()).then_some(VariantDef { idx, adt_def: *self })
    }

    pub fn repr(&self) -> ReprOptions {
        with(|cx| cx.adt_repr(*self))
    }

    pub fn discriminant_for_variant(&self, idx: VariantIdx) -> Discr {
        with(|cx| cx.adt_discr_for_variant(*self, idx))
    }
}
```

## Block 60
**Metadata**: AST_ID=60 | TYPE=STRUCT | NAME=Discr | COMPLEXITY=2 | LINES=5

```rust
pub struct Discr {
    pub val: u128,
    pub ty: Ty,
}
```

## Block 61
**Metadata**: AST_ID=61 | TYPE=STRUCT | NAME=VariantDef | COMPLEXITY=3 | LINES=17

```rust
/// Definition of a variant, which can be either a struct / union field or an enum variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct VariantDef {
    /// The variant index.
    ///
    /// ## Warning
    /// Do not access this field directly!
    pub idx: VariantIdx,
    /// The data type where this variant comes from.
    /// For now, we use this to retrieve information about the variant itself so we don't need to
    /// cache more information.
    ///
    /// ## Warning
    /// Do not access this field directly!
    pub adt_def: AdtDef,
}
```

## Block 62
**Metadata**: AST_ID=62 | TYPE=FUNCTION | NAME=name | COMPLEXITY=4 | LINES=13

```rust
impl VariantDef {
    pub fn name(&self) -> Symbol {
        with(|cx| cx.variant_name(*self))
    }

    /// Retrieve all the fields in this variant.
    // We expect user to cache this and use it directly since today it is expensive to generate all
    // fields name.
    pub fn fields(&self) -> Vec<FieldDef> {
        with(|cx| cx.variant_fields(*self))
    }
}
```

## Block 63
**Metadata**: AST_ID=63 | TYPE=STRUCT | NAME=FieldDef | COMPLEXITY=4 | LINES=12

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FieldDef {
    /// The field definition.
    ///
    /// ## Warning
    /// Do not access this field directly! This is public for the compiler to have access to it.
    pub def: DefId,

    /// The field name.
    pub name: Symbol,
}
```

## Block 64
**Metadata**: AST_ID=64 | TYPE=FUNCTION | NAME=ty_with_args | COMPLEXITY=4 | LINES=14

```rust
impl FieldDef {
    /// Retrieve the type of this field instantiating and normalizing it with the given arguments.
    ///
    /// This will assume the type can be instantiated with these arguments.
    pub fn ty_with_args(&self, args: &GenericArgs) -> Ty {
        with(|cx| cx.def_ty_with_args(self.def, args))
    }

    /// Retrieve the type of this field.
    pub fn ty(&self) -> Ty {
        with(|cx| cx.def_ty(self.def))
    }
}
```

## Block 65
**Metadata**: AST_ID=65 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=9 | LINES=10

```rust
impl Display for AdtKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            AdtKind::Enum => "enum",
            AdtKind::Union => "union",
            AdtKind::Struct => "struct",
        })
    }
}
```

## Block 66
**Metadata**: AST_ID=66 | TYPE=FUNCTION | NAME=is_enum | COMPLEXITY=5 | LINES=14

```rust
impl AdtKind {
    pub fn is_enum(&self) -> bool {
        matches!(self, AdtKind::Enum)
    }

    pub fn is_struct(&self) -> bool {
        matches!(self, AdtKind::Struct)
    }

    pub fn is_union(&self) -> bool {
        matches!(self, AdtKind::Union)
    }
}
```

## Block 67
**Metadata**: AST_ID=67 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
crate_def! {
    #[derive(Serialize)]
    pub AliasDef;
}
```

## Block 68
**Metadata**: AST_ID=68 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
crate_def! {
    /// A trait's definition.
    #[derive(Serialize)]
    pub TraitDef;
}
```

## Block 69
**Metadata**: AST_ID=69 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=4

```rust
impl_crate_def_items! {
    TraitDef;
}
```

## Block 70
**Metadata**: AST_ID=70 | TYPE=FUNCTION | NAME=declaration | COMPLEXITY=3 | LINES=6

```rust
impl TraitDef {
    pub fn declaration(trait_def: &TraitDef) -> TraitDecl {
        with(|cx| cx.trait_decl(trait_def))
    }
}
```

## Block 71
**Metadata**: AST_ID=71 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
crate_def! {
    #[derive(Serialize)]
    pub GenericDef;
}
```

## Block 72
**Metadata**: AST_ID=72 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
crate_def_with_ty! {
    #[derive(Serialize)]
    pub ConstDef;
}
```

## Block 73
**Metadata**: AST_ID=73 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
crate_def! {
    /// A trait impl definition.
    #[derive(Serialize)]
    pub ImplDef;
}
```

## Block 74
**Metadata**: AST_ID=74 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=4

```rust
impl_crate_def_items! {
    ImplDef;
}
```

## Block 75
**Metadata**: AST_ID=75 | TYPE=FUNCTION | NAME=trait_impl | COMPLEXITY=3 | LINES=7

```rust
impl ImplDef {
    /// Retrieve information about this implementation.
    pub fn trait_impl(&self) -> ImplTrait {
        with(|cx| cx.trait_impl(self))
    }
}
```

## Block 76
**Metadata**: AST_ID=76 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
crate_def! {
    #[derive(Serialize)]
    pub RegionDef;
}
```

## Block 77
**Metadata**: AST_ID=77 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
crate_def! {
    #[derive(Serialize)]
    pub CoroutineWitnessDef;
}
```

## Block 78
**Metadata**: AST_ID=78 | TYPE=FUNCTION | NAME=GenericArgs(pub | COMPLEXITY=5 | LINES=12

```rust
/// A list of generic arguments.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub struct GenericArgs(pub Vec<GenericArgKind>);

impl std::ops::Index<ParamTy> for GenericArgs {
    type Output = Ty;

    fn index(&self, index: ParamTy) -> &Self::Output {
        self.0[index.index as usize].expect_ty()
    }
}
```

## Block 79
**Metadata**: AST_ID=79 | TYPE=FUNCTION | NAME=index | COMPLEXITY=5 | LINES=8

```rust
impl std::ops::Index<ParamConst> for GenericArgs {
    type Output = TyConst;

    fn index(&self, index: ParamConst) -> &Self::Output {
        self.0[index.index as usize].expect_const()
    }
}
```

## Block 80
**Metadata**: AST_ID=80 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub enum GenericArgKind {
    Lifetime(Region),
    Type(Ty),
    Const(TyConst),
}
```

## Block 81
**Metadata**: AST_ID=81 | TYPE=FUNCTION | NAME=expect_ty | COMPLEXITY=26 | LINES=30

```rust
impl GenericArgKind {
    /// Panic if this generic argument is not a type, otherwise
    /// return the type.
    #[track_caller]
    pub fn expect_ty(&self) -> &Ty {
        match self {
            GenericArgKind::Type(ty) => ty,
            _ => panic!("{self:?}"),
        }
    }

    /// Panic if this generic argument is not a const, otherwise
    /// return the const.
    #[track_caller]
    pub fn expect_const(&self) -> &TyConst {
        match self {
            GenericArgKind::Const(c) => c,
            _ => panic!("{self:?}"),
        }
    }

    /// Return the generic argument type if applicable, otherwise return `None`.
    pub fn ty(&self) -> Option<&Ty> {
        match self {
            GenericArgKind::Type(ty) => Some(ty),
            _ => None,
        }
    }
}
```

## Block 82
**Metadata**: AST_ID=82 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum TermKind {
    Type(Ty),
    Const(TyConst),
}
```

## Block 83
**Metadata**: AST_ID=83 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum AliasKind {
    Projection,
    Inherent,
    Opaque,
    Free,
}
```

## Block 84
**Metadata**: AST_ID=84 | TYPE=STRUCT | NAME=AliasTy | COMPLEXITY=2 | LINES=6

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AliasTy {
    pub def_id: AliasDef,
    pub args: GenericArgs,
}
```

## Block 85
**Metadata**: AST_ID=85 | TYPE=STRUCT | NAME=AliasTerm | COMPLEXITY=2 | LINES=6

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AliasTerm {
    pub def_id: AliasDef,
    pub args: GenericArgs,
}
```

## Block 86
**Metadata**: AST_ID=86 | TYPE=FUNCTION | NAME=fn_ptr_abi | COMPLEXITY=5 | LINES=12

```rust
pub type PolyFnSig = Binder<FnSig>;

impl PolyFnSig {
    /// Compute a `FnAbi` suitable for indirect calls, i.e. to `fn` pointers.
    ///
    /// NB: this doesn't handle virtual calls - those should use `Instance::fn_abi`
    /// instead, where the instance is an `InstanceKind::Virtual`.
    pub fn fn_ptr_abi(self) -> Result<FnAbi, Error> {
        with(|cx| cx.fn_ptr_abi(self))
    }
}
```

## Block 87
**Metadata**: AST_ID=87 | TYPE=STRUCT | NAME=FnSig | COMPLEXITY=2 | LINES=8

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FnSig {
    pub inputs_and_output: Vec<Ty>,
    pub c_variadic: bool,
    pub safety: Safety,
    pub abi: Abi,
}
```

## Block 88
**Metadata**: AST_ID=88 | TYPE=FUNCTION | NAME=output | COMPLEXITY=4 | LINES=10

```rust
impl FnSig {
    pub fn output(&self) -> Ty {
        self.inputs_and_output[self.inputs_and_output.len() - 1]
    }

    pub fn inputs(&self) -> &[Ty] {
        &self.inputs_and_output[..self.inputs_and_output.len() - 1]
    }
}
```

## Block 89
**Metadata**: AST_ID=89 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=13 | LINES=31

```rust
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub enum Abi {
    Rust,
    C { unwind: bool },
    Cdecl { unwind: bool },
    Stdcall { unwind: bool },
    Fastcall { unwind: bool },
    Vectorcall { unwind: bool },
    Thiscall { unwind: bool },
    Aapcs { unwind: bool },
    Win64 { unwind: bool },
    SysV64 { unwind: bool },
    PtxKernel,
    Msp430Interrupt,
    X86Interrupt,
    GpuKernel,
    EfiApi,
    AvrInterrupt,
    AvrNonBlockingInterrupt,
    CCmseNonSecureCall,
    CCmseNonSecureEntry,
    System { unwind: bool },
    RustCall,
    Unadjusted,
    RustCold,
    RiscvInterruptM,
    RiscvInterruptS,
    RustInvalid,
    Custom,
}
```

## Block 90
**Metadata**: AST_ID=90 | TYPE=STRUCT | NAME=Binder | COMPLEXITY=2 | LINES=7

```rust
/// A binder represents a possibly generic type and its bound vars.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Binder<T> {
    pub value: T,
    pub bound_vars: Vec<BoundVariableKind>,
}
```

## Block 91
**Metadata**: AST_ID=91 | TYPE=FUNCTION | NAME=bind_with_vars | COMPLEXITY=14 | LINES=34

```rust
impl<T> Binder<T> {
    /// Create a new binder with the given bound vars.
    pub fn bind_with_vars(value: T, bound_vars: Vec<BoundVariableKind>) -> Self {
        Binder { value, bound_vars }
    }

    /// Create a new binder with no bounded variable.
    pub fn dummy(value: T) -> Self {
        Binder { value, bound_vars: vec![] }
    }

    pub fn skip_binder(self) -> T {
        self.value
    }

    pub fn map_bound_ref<F, U>(&self, f: F) -> Binder<U>
    where
        F: FnOnce(&T) -> U,
    {
        let Binder { value, bound_vars } = self;
        let new_value = f(value);
        Binder { value: new_value, bound_vars: bound_vars.clone() }
    }

    pub fn map_bound<F, U>(self, f: F) -> Binder<U>
    where
        F: FnOnce(T) -> U,
    {
        let Binder { value, bound_vars } = self;
        let new_value = f(value);
        Binder { value: new_value, bound_vars }
    }
}
```

## Block 92
**Metadata**: AST_ID=92 | TYPE=STRUCT | NAME=EarlyBinder | COMPLEXITY=2 | LINES=5

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct EarlyBinder<T> {
    pub value: T,
}
```

## Block 93
**Metadata**: AST_ID=93 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum BoundVariableKind {
    Ty(BoundTyKind),
    Region(BoundRegionKind),
    Const,
}
```

## Block 94
**Metadata**: AST_ID=94 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub enum BoundTyKind {
    Anon,
    Param(ParamDef, String),
}
```

## Block 95
**Metadata**: AST_ID=95 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub enum BoundRegionKind {
    BrAnon,
    BrNamed(BrNamedDef, String),
    BrEnv,
}
```

## Block 96
**Metadata**: AST_ID=96 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum DynKind {
    Dyn,
}
```

## Block 97
**Metadata**: AST_ID=97 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum ExistentialPredicate {
    Trait(ExistentialTraitRef),
    Projection(ExistentialProjection),
    AutoTrait(TraitDef),
}
```

## Block 98
**Metadata**: AST_ID=98 | TYPE=STRUCT | NAME=ExistentialTraitRef | COMPLEXITY=2 | LINES=9

```rust
/// An existential reference to a trait where `Self` is not included.
///
/// The `generic_args` will include any other known argument.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExistentialTraitRef {
    pub def_id: TraitDef,
    pub generic_args: GenericArgs,
}
```

## Block 99
**Metadata**: AST_ID=99 | TYPE=FUNCTION | NAME=with_self_ty | COMPLEXITY=3 | LINES=6

```rust
impl Binder<ExistentialTraitRef> {
    pub fn with_self_ty(&self, self_ty: Ty) -> Binder<TraitRef> {
        self.map_bound_ref(|trait_ref| trait_ref.with_self_ty(self_ty))
    }
}
```

## Block 100
**Metadata**: AST_ID=100 | TYPE=FUNCTION | NAME=with_self_ty | COMPLEXITY=3 | LINES=6

```rust
impl ExistentialTraitRef {
    pub fn with_self_ty(&self, self_ty: Ty) -> TraitRef {
        TraitRef::new(self.def_id, self_ty, &self.generic_args)
    }
}
```

## Block 101
**Metadata**: AST_ID=101 | TYPE=STRUCT | NAME=ExistentialProjection | COMPLEXITY=2 | LINES=7

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExistentialProjection {
    pub def_id: TraitDef,
    pub generic_args: GenericArgs,
    pub term: TermKind,
}
```

## Block 102
**Metadata**: AST_ID=102 | TYPE=STRUCT | NAME=ParamTy | COMPLEXITY=2 | LINES=6

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ParamTy {
    pub index: u32,
    pub name: String,
}
```

## Block 103
**Metadata**: AST_ID=103 | TYPE=STRUCT | NAME=BoundTy | COMPLEXITY=2 | LINES=6

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BoundTy {
    pub var: usize,
    pub kind: BoundTyKind,
}
```

## Block 104
**Metadata**: AST_ID=104 | TYPE=STRUCT | NAME=Prov(pub | COMPLEXITY=5 | LINES=20

```rust
pub type Bytes = Vec<Option<u8>>;

/// Size in bytes.
pub type Size = usize;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Serialize)]
pub struct Prov(pub AllocId);

pub type Align = u64;
pub type Promoted = u32;
pub type InitMaskMaterialized = Vec<u64>;

/// Stores the provenance information of pointers stored in memory.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub struct ProvenanceMap {
    /// Provenance in this map applies from the given offset for an entire pointer-size worth of
    /// bytes. Two entries in this map are always at least a pointer size apart.
    pub ptrs: Vec<(Size, Prov)>,
}
```

## Block 105
**Metadata**: AST_ID=105 | TYPE=STRUCT | NAME=Allocation | COMPLEXITY=2 | LINES=8

```rust
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub struct Allocation {
    pub bytes: Bytes,
    pub provenance: ProvenanceMap,
    pub align: Align,
    pub mutability: Mutability,
}
```

## Block 106
**Metadata**: AST_ID=106 | TYPE=FUNCTION | NAME=raw_bytes | COMPLEXITY=43 | LINES=68

```rust
impl Allocation {
    /// Get a vector of bytes for an Allocation that has been fully initialized
    pub fn raw_bytes(&self) -> Result<Vec<u8>, Error> {
        self.bytes
            .iter()
            .copied()
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| error!("Found uninitialized bytes: `{:?}`", self.bytes))
    }

    /// Read a uint value from the specified range.
    pub fn read_partial_uint(&self, range: Range<usize>) -> Result<u128, Error> {
        if range.end - range.start > 16 {
            return Err(error!("Allocation is bigger than largest integer"));
        }
        if range.end > self.bytes.len() {
            return Err(error!(
                "Range is out of bounds. Allocation length is `{}`, but requested range `{:?}`",
                self.bytes.len(),
                range
            ));
        }
        let raw = self.bytes[range]
            .iter()
            .copied()
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| error!("Found uninitialized bytes: `{:?}`", self.bytes))?;
        read_target_uint(&raw)
    }

    /// Read this allocation and try to convert it to an unassigned integer.
    pub fn read_uint(&self) -> Result<u128, Error> {
        if self.bytes.len() > 16 {
            return Err(error!("Allocation is bigger than largest integer"));
        }
        let raw = self.raw_bytes()?;
        read_target_uint(&raw)
    }

    /// Read this allocation and try to convert it to a signed integer.
    pub fn read_int(&self) -> Result<i128, Error> {
        if self.bytes.len() > 16 {
            return Err(error!("Allocation is bigger than largest integer"));
        }
        let raw = self.raw_bytes()?;
        read_target_int(&raw)
    }

    /// Read this allocation and try to convert it to a boolean.
    pub fn read_bool(&self) -> Result<bool, Error> {
        match self.read_int()? {
            0 => Ok(false),
            1 => Ok(true),
            val => Err(error!("Unexpected value for bool: `{val}`")),
        }
    }

    /// Read this allocation as a pointer and return whether it represents a `null` pointer.
    pub fn is_null(&self) -> Result<bool, Error> {
        let len = self.bytes.len();
        let ptr_len = MachineInfo::target_pointer_width().bytes();
        if len != ptr_len {
            return Err(error!("Expected width of pointer (`{ptr_len}`), but found: `{len}`"));
        }
        Ok(self.read_uint()? == 0 && self.provenance.ptrs.is_empty())
    }
}
```

## Block 107
**Metadata**: AST_ID=107 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=11

```rust
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub enum ConstantKind {
    Ty(TyConst),
    Allocated(Allocation),
    Unevaluated(UnevaluatedConst),
    Param(ParamConst),
    /// Store ZST constants.
    /// We have to special handle these constants since its type might be generic.
    ZeroSized,
}
```

## Block 108
**Metadata**: AST_ID=108 | TYPE=STRUCT | NAME=ParamConst | COMPLEXITY=2 | LINES=6

```rust
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub struct ParamConst {
    pub index: u32,
    pub name: String,
}
```

## Block 109
**Metadata**: AST_ID=109 | TYPE=STRUCT | NAME=UnevaluatedConst | COMPLEXITY=2 | LINES=7

```rust
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub struct UnevaluatedConst {
    pub def: ConstDef,
    pub args: GenericArgs,
    pub promoted: Option<Promoted>,
}
```

## Block 110
**Metadata**: AST_ID=110 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum TraitSpecializationKind {
    None,
    Marker,
    AlwaysApplicable,
}
```

## Block 111
**Metadata**: AST_ID=111 | TYPE=STRUCT | NAME=TraitDecl | COMPLEXITY=3 | LINES=16

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TraitDecl {
    pub def_id: TraitDef,
    pub safety: Safety,
    pub paren_sugar: bool,
    pub has_auto_impl: bool,
    pub is_marker: bool,
    pub is_coinductive: bool,
    pub skip_array_during_method_dispatch: bool,
    pub skip_boxed_slice_during_method_dispatch: bool,
    pub specialization_kind: TraitSpecializationKind,
    pub must_implement_one_of: Option<Vec<Ident>>,
    pub implement_via_object: bool,
    pub deny_explicit_impl: bool,
}
```

## Block 112
**Metadata**: AST_ID=112 | TYPE=FUNCTION | NAME=generics_of | COMPLEXITY=5 | LINES=14

```rust
impl TraitDecl {
    pub fn generics_of(&self) -> Generics {
        with(|cx| cx.generics_of(self.def_id.0))
    }

    pub fn predicates_of(&self) -> GenericPredicates {
        with(|cx| cx.predicates_of(self.def_id.0))
    }

    pub fn explicit_predicates_of(&self) -> GenericPredicates {
        with(|cx| cx.explicit_predicates_of(self.def_id.0))
    }
}
```

## Block 113
**Metadata**: AST_ID=113 | TYPE=STRUCT | NAME=TraitRef | COMPLEXITY=4 | LINES=11

```rust
pub type ImplTrait = EarlyBinder<TraitRef>;

/// A complete reference to a trait, i.e., one where `Self` is known.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TraitRef {
    pub def_id: TraitDef,
    /// The generic arguments for this definition.
    /// The first element must always be type, and it represents `Self`.
    args: GenericArgs,
}
```

## Block 114
**Metadata**: AST_ID=114 | TYPE=FUNCTION | NAME=new | COMPLEXITY=15 | LINES=26

```rust
impl TraitRef {
    pub fn new(def_id: TraitDef, self_ty: Ty, gen_args: &GenericArgs) -> TraitRef {
        let mut args = vec![GenericArgKind::Type(self_ty)];
        args.extend_from_slice(&gen_args.0);
        TraitRef { def_id, args: GenericArgs(args) }
    }

    pub fn try_new(def_id: TraitDef, args: GenericArgs) -> Result<TraitRef, ()> {
        match &args.0[..] {
            [GenericArgKind::Type(_), ..] => Ok(TraitRef { def_id, args }),
            _ => Err(()),
        }
    }

    pub fn args(&self) -> &GenericArgs {
        &self.args
    }

    pub fn self_ty(&self) -> Ty {
        let GenericArgKind::Type(self_ty) = self.args.0[0] else {
            panic!("Self must be a type, but found: {:?}", self.args.0[0])
        };
        self_ty
    }
}
```

## Block 115
**Metadata**: AST_ID=115 | TYPE=STRUCT | NAME=Generics | COMPLEXITY=2 | LINES=10

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Generics {
    pub parent: Option<GenericDef>,
    pub parent_count: usize,
    pub params: Vec<GenericParamDef>,
    pub param_def_id_to_index: Vec<(GenericDef, u32)>,
    pub has_self: bool,
    pub has_late_bound_regions: Option<Span>,
}
```

## Block 116
**Metadata**: AST_ID=116 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=4 | LINES=7

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum GenericParamDefKind {
    Lifetime,
    Type { has_default: bool, synthetic: bool },
    Const { has_default: bool },
}
```

## Block 117
**Metadata**: AST_ID=117 | TYPE=STRUCT | NAME=GenericParamDef | COMPLEXITY=2 | LINES=9

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct GenericParamDef {
    pub name: super::Symbol,
    pub def_id: GenericDef,
    pub index: u32,
    pub pure_wrt_drop: bool,
    pub kind: GenericParamDefKind,
}
```

## Block 118
**Metadata**: AST_ID=118 | TYPE=STRUCT | NAME=GenericPredicates | COMPLEXITY=2 | LINES=5

```rust
pub struct GenericPredicates {
    pub parent: Option<TraitDef>,
    pub predicates: Vec<(PredicateKind, Span)>,
}
```

## Block 119
**Metadata**: AST_ID=119 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=11

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum PredicateKind {
    Clause(ClauseKind),
    DynCompatible(TraitDef),
    SubType(SubtypePredicate),
    Coerce(CoercePredicate),
    ConstEquate(TyConst, TyConst),
    Ambiguous,
    AliasRelate(TermKind, TermKind, AliasRelationDirection),
}
```

## Block 120
**Metadata**: AST_ID=120 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=11

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum ClauseKind {
    Trait(TraitPredicate),
    RegionOutlives(RegionOutlivesPredicate),
    TypeOutlives(TypeOutlivesPredicate),
    Projection(ProjectionPredicate),
    ConstArgHasType(TyConst, Ty),
    WellFormed(TermKind),
    ConstEvaluatable(TyConst),
}
```

## Block 121
**Metadata**: AST_ID=121 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum ClosureKind {
    Fn,
    FnMut,
    FnOnce,
}
```

## Block 122
**Metadata**: AST_ID=122 | TYPE=STRUCT | NAME=SubtypePredicate | COMPLEXITY=2 | LINES=6

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SubtypePredicate {
    pub a: Ty,
    pub b: Ty,
}
```

## Block 123
**Metadata**: AST_ID=123 | TYPE=STRUCT | NAME=CoercePredicate | COMPLEXITY=2 | LINES=6

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CoercePredicate {
    pub a: Ty,
    pub b: Ty,
}
```

## Block 124
**Metadata**: AST_ID=124 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum AliasRelationDirection {
    Equate,
    Subtype,
}
```

## Block 125
**Metadata**: AST_ID=125 | TYPE=STRUCT | NAME=TraitPredicate | COMPLEXITY=2 | LINES=6

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TraitPredicate {
    pub trait_ref: TraitRef,
    pub polarity: PredicatePolarity,
}
```

## Block 126
**Metadata**: AST_ID=126 | TYPE=STRUCT | NAME=OutlivesPredicate | COMPLEXITY=2 | LINES=12

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OutlivesPredicate<A, B>(pub A, pub B);

pub type RegionOutlivesPredicate = OutlivesPredicate<Region, Region>;
pub type TypeOutlivesPredicate = OutlivesPredicate<Ty, Region>;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProjectionPredicate {
    pub projection_term: AliasTerm,
    pub term: TermKind,
}
```

## Block 127
**Metadata**: AST_ID=127 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum ImplPolarity {
    Positive,
    Negative,
    Reservation,
}
```

## Block 128
**Metadata**: AST_ID=128 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum PredicatePolarity {
    Positive,
    Negative,
}
```

## Block 129
**Metadata**: AST_ID=129 | TYPE=FUNCTION | NAME=to_val | COMPLEXITY=13 | LINES=13

```rust
macro_rules! index_impl {
    ($name:ident) => {
        impl crate::IndexedVal for $name {
            fn to_val(index: usize) -> Self {
                $name(index)
            }
            fn to_index(&self) -> usize {
                self.0
            }
        }
    };
}
```

## Block 130
**Metadata**: AST_ID=130 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=4 | LINES=14

```rust
index_impl!(TyConstId);
index_impl!(MirConstId);
index_impl!(Ty);
index_impl!(Span);

/// The source-order index of a variant in a type.
///
/// For example, in the following types,
/// ```ignore(illustrative)
/// enum Demo1 {
///    Variant0 { a: bool, b: i32 },
///    Variant1 { c: u8, d: u64 },
/// }
```

## Block 131
**Metadata**: AST_ID=131 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
/// struct Demo2 { e: u8, f: u16, g: u8 }
```

## Block 132
**Metadata**: AST_ID=132 | TYPE=STRUCT | NAME=VariantIdx(usize); | COMPLEXITY=2 | LINES=14

```rust
/// ```
/// `a` is in the variant with the `VariantIdx` of `0`,
/// `c` is in the variant with the `VariantIdx` of `1`, and
/// `g` is in the variant with the `VariantIdx` of `0`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct VariantIdx(usize);

index_impl!(VariantIdx);

crate_def! {
    /// Hold information about an Opaque definition, particularly useful in `RPITIT`.
    #[derive(Serialize)]
    pub OpaqueDef;
}
```

## Block 133
**Metadata**: AST_ID=133 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
crate_def! {
    #[derive(Serialize)]
    pub AssocDef;
}
```

## Block 134
**Metadata**: AST_ID=134 | TYPE=STRUCT | NAME=AssocItem | COMPLEXITY=2 | LINES=7

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AssocItem {
    pub def_id: AssocDef,
    pub kind: AssocKind,
    pub container: AssocContainer,
}
```

## Block 135
**Metadata**: AST_ID=135 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Clone, PartialEq, Debug, Eq, Serialize)]
pub enum AssocTypeData {
    Normal(Symbol),
    /// The associated type comes from an RPITIT. It has no name, and the
    /// `ImplTraitInTraitData` provides additional information about its
    /// source.
    Rpitit(ImplTraitInTraitData),
}
```

## Block 136
**Metadata**: AST_ID=136 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=5 | LINES=7

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum AssocKind {
    Const { name: Symbol },
    Fn { name: Symbol, has_self: bool },
    Type { data: AssocTypeData },
}
```

## Block 137
**Metadata**: AST_ID=137 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum AssocContainer {
    InherentImpl,
    /// The `AssocDef` points to the trait item being implemented.
    TraitImpl(AssocDef),
    Trait,
}
```

## Block 138
**Metadata**: AST_ID=138 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=4 | LINES=6

```rust
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Serialize)]
pub enum ImplTraitInTraitData {
    Trait { fn_def_id: FnDef, opaque_def_id: OpaqueDef },
    Impl { fn_def_id: FnDef },
}
```

## Block 139
**Metadata**: AST_ID=139 | TYPE=FUNCTION | NAME=is_impl_trait_in_trait | COMPLEXITY=4 | LINES=6

```rust
impl AssocItem {
    pub fn is_impl_trait_in_trait(&self) -> bool {
        matches!(self.kind, AssocKind::Type { data: AssocTypeData::Rpitit(_) })
    }
}
```

---
*Generated by AST tracing system*
