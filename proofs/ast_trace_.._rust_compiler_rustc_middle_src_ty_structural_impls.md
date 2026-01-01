# AST Trace: ../rust/compiler/rustc_middle/src/ty/structural_impls.rs

Generated 61 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=4 | LINES=6

```rust
// This module contains implementations of the `Lift`, `TypeFoldable` and
// `TypeVisitable` traits for various types in the Rust compiler. Most are
// written by hand, though we've recently added some macros and proc-macros
// to help with the tedium.

use std::fmt::{self, Debug};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
use std::marker::PhantomData;

use crate::rustc_abi::TyAndLayout;
use crate::rustc_complete::def::Namespace;
use crate::rustc_complete::def_id::LocalDefId;
use crate::rustc_complete::source_map::Spanned;
use rustc_type_ir::{ConstKind, TypeFolder, VisitorResult, try_visit};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use super::{GenericArg, GenericArgKind, Pattern, Region};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use crate::mir::PlaceElem;
use crate::ty::print::{FmtPrinter, Printer, with_no_trimmed_paths};
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4

```rust
use crate::ty::{
    self, FallibleTypeFolder, Lift, Term, TermKind, Ty, TyCtxt, TypeFoldable, TypeSuperFoldable,
    TypeSuperVisitable, TypeVisitable, TypeVisitor,
};
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=8 | LINES=13

```rust
impl fmt::Debug for ty::TraitDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        ty::tls::with(|tcx| {
            with_no_trimmed_paths!({
                let s = FmtPrinter::print_string(tcx, Namespace::TypeNS, |p| {
                    p.print_def_path(self.def_id, &[])
                })?;
                f.write_str(&s)
            })
        })
    }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=8 | LINES=13

```rust
impl<'tcx> fmt::Debug for ty::AdtDef<'tcx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        ty::tls::with(|tcx| {
            with_no_trimmed_paths!({
                let s = FmtPrinter::print_string(tcx, Namespace::TypeNS, |p| {
                    p.print_def_path(self.did(), &[])
                })?;
                f.write_str(&s)
            })
        })
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=8 | LINES=7

```rust
impl fmt::Debug for ty::UpvarId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = ty::tls::with(|tcx| tcx.hir_name(self.var_path.hir_id));
        write!(f, "UpvarId({:?};`{}`;{:?})", self.var_path.hir_id, name, self.closure_expr_id)
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=7 | LINES=6

```rust
impl<'tcx> fmt::Debug for ty::adjustment::Adjustment<'tcx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} -> {}", self.kind, self.target)
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=7 | LINES=6

```rust
impl<'tcx> fmt::Debug for ty::adjustment::PatAdjustment<'tcx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {:?}", self.source, self.kind)
    }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=14 | LINES=15

```rust
impl fmt::Debug for ty::BoundRegionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ty::BoundRegionKind::Anon => write!(f, "BrAnon"),
            ty::BoundRegionKind::NamedAnon(name) => {
                write!(f, "BrNamedAnon({name})")
            }
            ty::BoundRegionKind::Named(did) => {
                write!(f, "BrNamed({did:?})")
            }
            ty::BoundRegionKind::ClosureEnv => write!(f, "BrEnv"),
        }
    }
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=7 | LINES=6

```rust
impl fmt::Debug for ty::LateParamRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ReLateParam({:?}, {:?})", self.scope, self.kind)
    }
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=16 | LINES=15

```rust
impl fmt::Debug for ty::LateParamRegionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ty::LateParamRegionKind::Anon(idx) => write!(f, "LateAnon({idx})"),
            ty::LateParamRegionKind::NamedAnon(idx, name) => {
                write!(f, "LateNamedAnon({idx:?}, {name})")
            }
            ty::LateParamRegionKind::Named(did) => {
                write!(f, "LateNamed({did:?})")
            }
            ty::LateParamRegionKind::ClosureEnv => write!(f, "LateEnv"),
        }
    }
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx> fmt::Debug for Ty<'tcx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        with_no_trimmed_paths!(fmt::Debug::fmt(self.kind(), f))
    }
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=7 | LINES=6

```rust
impl fmt::Debug for ty::ParamTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/#{}", self.name, self.index)
    }
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=7 | LINES=6

```rust
impl fmt::Debug for ty::ParamConst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/#{}", self.name, self.index)
    }
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=6 | LINES=6

```rust
impl<'tcx> fmt::Debug for ty::Predicate<'tcx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.kind())
    }
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=6 | LINES=6

```rust
impl<'tcx> fmt::Debug for ty::Clause<'tcx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.kind())
    }
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=37 | LINES=32

```rust
impl<'tcx> fmt::Debug for ty::consts::Expr<'tcx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ty::ExprKind::Binop(op) => {
                let (lhs_ty, rhs_ty, lhs, rhs) = self.binop_args();
                write!(f, "({op:?}: ({:?}: {:?}), ({:?}: {:?}))", lhs, lhs_ty, rhs, rhs_ty,)
            }
            ty::ExprKind::UnOp(op) => {
                let (rhs_ty, rhs) = self.unop_args();
                write!(f, "({op:?}: ({:?}: {:?}))", rhs, rhs_ty)
            }
            ty::ExprKind::FunctionCall => {
                let (func_ty, func, args) = self.call_args();
                let args = args.collect::<Vec<_>>();
                write!(f, "({:?}: {:?})(", func, func_ty)?;
                for arg in args.iter().rev().skip(1).rev() {
                    write!(f, "{:?}, ", arg)?;
                }
                if let Some(arg) = args.last() {
                    write!(f, "{:?}", arg)?;
                }

                write!(f, ")")
            }
            ty::ExprKind::Cast(kind) => {
                let (value_ty, value, to_ty) = self.cast_args();
                write!(f, "({kind:?}: ({:?}: {:?}), {:?})", value, value_ty, to_ty)
            }
        }
    }
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=11 | LINES=12

```rust
impl<'tcx> fmt::Debug for ty::Const<'tcx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // If this is a value, we spend some effort to make it look nice.
        if let ConstKind::Value(cv) = self.kind() {
            write!(f, "{}", cv)
        } else {
            // Fall back to something verbose.
            write!(f, "{:?}", self.kind())
        }
    }
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=11 | LINES=9

```rust
impl fmt::Debug for ty::BoundTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ty::BoundTyKind::Anon => write!(f, "{:?}", self.var),
            ty::BoundTyKind::Param(def_id) => write!(f, "{def_id:?}"),
        }
    }
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=12 | LINES=10

```rust
impl<T: fmt::Debug> fmt::Debug for ty::Placeholder<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.universe == ty::UniverseIndex::ROOT {
            write!(f, "!{:?}", self.bound)
        } else {
            write!(f, "!{}_{:?}", self.universe.index(), self.bound)
        }
    }
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=9 | LINES=10

```rust
impl<'tcx> fmt::Debug for GenericArg<'tcx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind() {
            GenericArgKind::Lifetime(lt) => lt.fmt(f),
            GenericArgKind::Type(ty) => ty.fmt(f),
            GenericArgKind::Const(ct) => ct.fmt(f),
        }
    }
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=6 | LINES=6

```rust
impl<'tcx> fmt::Debug for Region<'tcx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.kind())
    }
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=7 | LINES=27

```rust
///////////////////////////////////////////////////////////////////////////
// Atomic structs
//
// For things that don't carry any arena-allocated data (and are
// copy...), just add them to one of these lists as appropriate.

// For things for which the type library provides traversal implementations
// for all Interners, we only need to provide a Lift implementation.
TrivialLiftImpls! {
    (),
    bool,
    usize,
    u64,
    // tidy-alphabetical-start
    crate::mir::Promoted,
    crate::mir::interpret::AllocId,
    crate::mir::interpret::Scalar,
    crate::ty::ParamConst,
    crate::rustc_abi::ExternAbi,
    crate::rustc_abi::Size,
    crate::rustc_hir::Safety,
    crate::rustc_middle::mir::ConstValue,
    rustc_type_ir::BoundConstness,
    rustc_type_ir::PredicatePolarity,
    // tidy-alphabetical-end
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=7 | LINES=51

```rust
// For some things about which the type library does not know, or does not
// provide any traversal implementations, we need to provide a traversal
// implementation (only for TyCtxt<'_> interners).
TrivialTypeTraversalImpls! {
    // tidy-alphabetical-start
    crate::infer::canonical::Certainty,
    crate::mir::BasicBlock,
    crate::mir::BindingForm<'tcx>,
    crate::mir::BlockTailInfo,
    crate::mir::BorrowKind,
    crate::mir::CastKind,
    crate::mir::ConstValue,
    crate::mir::CoroutineSavedLocal,
    crate::mir::FakeReadCause,
    crate::mir::Local,
    crate::mir::MirPhase,
    crate::mir::NullOp<'tcx>,
    crate::mir::Promoted,
    crate::mir::RawPtrKind,
    crate::mir::RetagKind,
    crate::mir::SourceInfo,
    crate::mir::SourceScope,
    crate::mir::SourceScopeLocalData,
    crate::mir::SwitchTargets,
    crate::traits::IsConstable,
    crate::traits::OverflowError,
    crate::ty::AdtKind,
    crate::ty::AssocItem,
    crate::ty::AssocKind,
    crate::ty::BoundRegion,
    crate::ty::UserTypeAnnotationIndex,
    crate::ty::ValTree<'tcx>,
    crate::ty::abstract_const::NotConstEvaluatable,
    crate::ty::adjustment::AutoBorrowMutability,
    crate::ty::adjustment::PointerCoercion,
    crate::rustc_abi::FieldIdx,
    crate::rustc_abi::VariantIdx,
    crate::rustc_ast::InlineAsmOptions,
    crate::rustc_ast::InlineAsmTemplatePiece,
    crate::rustc_hir::CoroutineKind,
    crate::rustc_hir::HirId,
    crate::rustc_hir::MatchSource,
    crate::rustc_hir::RangeEnd,
    crate::rustc_hir::def_id::LocalDefId,
    crate::rustc_span::Ident,
    crate::rustc_span::Span,
    crate::rustc_span::Symbol,
    crate::rustc_target::asm::InlineAsmRegOrRegClass,
    // tidy-alphabetical-end
}
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=13

```rust
// For some things about which the type library does not know, or does not
// provide any traversal implementations, we need to provide a traversal
// implementation and a lift implementation (the former only for TyCtxt<'_>
// interners).
TrivialTypeTraversalAndLiftImpls! {
    // tidy-alphabetical-start
    crate::ty::ParamTy,
    crate::ty::PlaceholderType,
    crate::ty::instance::ReifyReason,
    crate::rustc_hir::def_id::DefId,
    // tidy-alphabetical-end
}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=FUNCTION | NAME=lift_to_interner | COMPLEXITY=5 | LINES=10

```rust
///////////////////////////////////////////////////////////////////////////
// Lift implementations

impl<'tcx> Lift<TyCtxt<'tcx>> for PhantomData<&()> {
    type Lifted = PhantomData<&'tcx ()>;
    fn lift_to_interner(self, _: TyCtxt<'tcx>) -> Option<Self::Lifted> {
        Some(PhantomData)
    }
}
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=FUNCTION | NAME=lift_to_interner | COMPLEXITY=9 | LINES=10

```rust
impl<'tcx, T: Lift<TyCtxt<'tcx>>> Lift<TyCtxt<'tcx>> for Option<T> {
    type Lifted = Option<T::Lifted>;
    fn lift_to_interner(self, tcx: TyCtxt<'tcx>) -> Option<Self::Lifted> {
        Some(match self {
            Some(x) => Some(tcx.lift(x)?),
            None => None,
        })
    }
}
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=FUNCTION | NAME=lift_to_interner | COMPLEXITY=9 | LINES=10

```rust
impl<'a, 'tcx> Lift<TyCtxt<'tcx>> for Term<'a> {
    type Lifted = ty::Term<'tcx>;
    fn lift_to_interner(self, tcx: TyCtxt<'tcx>) -> Option<Self::Lifted> {
        match self.kind() {
            TermKind::Ty(ty) => tcx.lift(ty).map(Into::into),
            TermKind::Const(c) => tcx.lift(c).map(Into::into),
        }
    }
}
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=FUNCTION | NAME=visit_with | COMPLEXITY=5 | LINES=9

```rust
///////////////////////////////////////////////////////////////////////////
// Traversal implementations.

impl<'tcx> TypeVisitable<TyCtxt<'tcx>> for ty::AdtDef<'tcx> {
    fn visit_with<V: TypeVisitor<TyCtxt<'tcx>>>(&self, _visitor: &mut V) -> V::Result {
        V::Result::output()
    }
}
```

## Block 32
**Metadata**: AST_ID=32 | TYPE=FUNCTION | NAME=try_fold_with | COMPLEXITY=15 | LINES=15

```rust
impl<'tcx> TypeFoldable<TyCtxt<'tcx>> for Pattern<'tcx> {
    fn try_fold_with<F: FallibleTypeFolder<TyCtxt<'tcx>>>(
        self,
        folder: &mut F,
    ) -> Result<Self, F::Error> {
        let pat = (*self).clone().try_fold_with(folder)?;
        Ok(if pat == *self { self } else { folder.cx().mk_pat(pat) })
    }

    fn fold_with<F: TypeFolder<TyCtxt<'tcx>>>(self, folder: &mut F) -> Self {
        let pat = (*self).clone().fold_with(folder);
        if pat == *self { self } else { folder.cx().mk_pat(pat) }
    }
}
```

## Block 33
**Metadata**: AST_ID=33 | TYPE=FUNCTION | NAME=visit_with | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx> TypeVisitable<TyCtxt<'tcx>> for Pattern<'tcx> {
    fn visit_with<V: TypeVisitor<TyCtxt<'tcx>>>(&self, visitor: &mut V) -> V::Result {
        (**self).visit_with(visitor)
    }
}
```

## Block 34
**Metadata**: AST_ID=34 | TYPE=FUNCTION | NAME=try_fold_with | COMPLEXITY=6 | LINES=13

```rust
impl<'tcx> TypeFoldable<TyCtxt<'tcx>> for Ty<'tcx> {
    fn try_fold_with<F: FallibleTypeFolder<TyCtxt<'tcx>>>(
        self,
        folder: &mut F,
    ) -> Result<Self, F::Error> {
        folder.try_fold_ty(self)
    }

    fn fold_with<F: TypeFolder<TyCtxt<'tcx>>>(self, folder: &mut F) -> Self {
        folder.fold_ty(self)
    }
}
```

## Block 35
**Metadata**: AST_ID=35 | TYPE=FUNCTION | NAME=visit_with | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx> TypeVisitable<TyCtxt<'tcx>> for Ty<'tcx> {
    fn visit_with<V: TypeVisitor<TyCtxt<'tcx>>>(&self, visitor: &mut V) -> V::Result {
        visitor.visit_ty(*self)
    }
}
```

## Block 36
**Metadata**: AST_ID=36 | TYPE=FUNCTION | NAME=try_super_fold_with | COMPLEXITY=34 | LINES=91

```rust
impl<'tcx> TypeSuperFoldable<TyCtxt<'tcx>> for Ty<'tcx> {
    fn try_super_fold_with<F: FallibleTypeFolder<TyCtxt<'tcx>>>(
        self,
        folder: &mut F,
    ) -> Result<Self, F::Error> {
        let kind = match *self.kind() {
            ty::RawPtr(ty, mutbl) => ty::RawPtr(ty.try_fold_with(folder)?, mutbl),
            ty::Array(typ, sz) => ty::Array(typ.try_fold_with(folder)?, sz.try_fold_with(folder)?),
            ty::Slice(typ) => ty::Slice(typ.try_fold_with(folder)?),
            ty::Adt(tid, args) => ty::Adt(tid, args.try_fold_with(folder)?),
            ty::Dynamic(trait_ty, region, representation) => ty::Dynamic(
                trait_ty.try_fold_with(folder)?,
                region.try_fold_with(folder)?,
                representation,
            ),
            ty::Tuple(ts) => ty::Tuple(ts.try_fold_with(folder)?),
            ty::FnDef(def_id, args) => ty::FnDef(def_id, args.try_fold_with(folder)?),
            ty::FnPtr(sig_tys, hdr) => ty::FnPtr(sig_tys.try_fold_with(folder)?, hdr),
            ty::UnsafeBinder(f) => ty::UnsafeBinder(f.try_fold_with(folder)?),
            ty::Ref(r, ty, mutbl) => {
                ty::Ref(r.try_fold_with(folder)?, ty.try_fold_with(folder)?, mutbl)
            }
            ty::Coroutine(did, args) => ty::Coroutine(did, args.try_fold_with(folder)?),
            ty::CoroutineWitness(did, args) => {
                ty::CoroutineWitness(did, args.try_fold_with(folder)?)
            }
            ty::Closure(did, args) => ty::Closure(did, args.try_fold_with(folder)?),
            ty::CoroutineClosure(did, args) => {
                ty::CoroutineClosure(did, args.try_fold_with(folder)?)
            }
            ty::Alias(kind, data) => ty::Alias(kind, data.try_fold_with(folder)?),
            ty::Pat(ty, pat) => ty::Pat(ty.try_fold_with(folder)?, pat.try_fold_with(folder)?),

            ty::Bool
            | ty::Char
            | ty::Str
            | ty::Int(_)
            | ty::Uint(_)
            | ty::Float(_)
            | ty::Error(_)
            | ty::Infer(_)
            | ty::Param(..)
            | ty::Bound(..)
            | ty::Placeholder(..)
            | ty::Never
            | ty::Foreign(..) => return Ok(self),
        };

        Ok(if *self.kind() == kind { self } else { folder.cx().mk_ty_from_kind(kind) })
    }

    fn super_fold_with<F: TypeFolder<TyCtxt<'tcx>>>(self, folder: &mut F) -> Self {
        let kind = match *self.kind() {
            ty::RawPtr(ty, mutbl) => ty::RawPtr(ty.fold_with(folder), mutbl),
            ty::Array(typ, sz) => ty::Array(typ.fold_with(folder), sz.fold_with(folder)),
            ty::Slice(typ) => ty::Slice(typ.fold_with(folder)),
            ty::Adt(tid, args) => ty::Adt(tid, args.fold_with(folder)),
            ty::Dynamic(trait_ty, region, representation) => {
                ty::Dynamic(trait_ty.fold_with(folder), region.fold_with(folder), representation)
            }
            ty::Tuple(ts) => ty::Tuple(ts.fold_with(folder)),
            ty::FnDef(def_id, args) => ty::FnDef(def_id, args.fold_with(folder)),
            ty::FnPtr(sig_tys, hdr) => ty::FnPtr(sig_tys.fold_with(folder), hdr),
            ty::UnsafeBinder(f) => ty::UnsafeBinder(f.fold_with(folder)),
            ty::Ref(r, ty, mutbl) => ty::Ref(r.fold_with(folder), ty.fold_with(folder), mutbl),
            ty::Coroutine(did, args) => ty::Coroutine(did, args.fold_with(folder)),
            ty::CoroutineWitness(did, args) => ty::CoroutineWitness(did, args.fold_with(folder)),
            ty::Closure(did, args) => ty::Closure(did, args.fold_with(folder)),
            ty::CoroutineClosure(did, args) => ty::CoroutineClosure(did, args.fold_with(folder)),
            ty::Alias(kind, data) => ty::Alias(kind, data.fold_with(folder)),
            ty::Pat(ty, pat) => ty::Pat(ty.fold_with(folder), pat.fold_with(folder)),

            ty::Bool
            | ty::Char
            | ty::Str
            | ty::Int(_)
            | ty::Uint(_)
            | ty::Float(_)
            | ty::Error(_)
            | ty::Infer(_)
            | ty::Param(..)
            | ty::Bound(..)
            | ty::Placeholder(..)
            | ty::Never
            | ty::Foreign(..) => return self,
        };

        if *self.kind() == kind { self } else { folder.cx().mk_ty_from_kind(kind) }
    }
}
```

## Block 37
**Metadata**: AST_ID=37 | TYPE=FUNCTION | NAME=super_visit_with | COMPLEXITY=16 | LINES=51

```rust
impl<'tcx> TypeSuperVisitable<TyCtxt<'tcx>> for Ty<'tcx> {
    fn super_visit_with<V: TypeVisitor<TyCtxt<'tcx>>>(&self, visitor: &mut V) -> V::Result {
        match self.kind() {
            ty::RawPtr(ty, _mutbl) => ty.visit_with(visitor),
            ty::Array(typ, sz) => {
                try_visit!(typ.visit_with(visitor));
                sz.visit_with(visitor)
            }
            ty::Slice(typ) => typ.visit_with(visitor),
            ty::Adt(_, args) => args.visit_with(visitor),
            ty::Dynamic(trait_ty, reg, _) => {
                try_visit!(trait_ty.visit_with(visitor));
                reg.visit_with(visitor)
            }
            ty::Tuple(ts) => ts.visit_with(visitor),
            ty::FnDef(_, args) => args.visit_with(visitor),
            ty::FnPtr(sig_tys, _) => sig_tys.visit_with(visitor),
            ty::UnsafeBinder(f) => f.visit_with(visitor),
            ty::Ref(r, ty, _) => {
                try_visit!(r.visit_with(visitor));
                ty.visit_with(visitor)
            }
            ty::Coroutine(_did, args) => args.visit_with(visitor),
            ty::CoroutineWitness(_did, args) => args.visit_with(visitor),
            ty::Closure(_did, args) => args.visit_with(visitor),
            ty::CoroutineClosure(_did, args) => args.visit_with(visitor),
            ty::Alias(_, data) => data.visit_with(visitor),

            ty::Pat(ty, pat) => {
                try_visit!(ty.visit_with(visitor));
                pat.visit_with(visitor)
            }

            ty::Error(guar) => guar.visit_with(visitor),

            ty::Bool
            | ty::Char
            | ty::Str
            | ty::Int(_)
            | ty::Uint(_)
            | ty::Float(_)
            | ty::Infer(_)
            | ty::Bound(..)
            | ty::Placeholder(..)
            | ty::Param(..)
            | ty::Never
            | ty::Foreign(..) => V::Result::output(),
        }
    }
}
```

## Block 38
**Metadata**: AST_ID=38 | TYPE=FUNCTION | NAME=try_fold_with | COMPLEXITY=6 | LINES=13

```rust
impl<'tcx> TypeFoldable<TyCtxt<'tcx>> for ty::Region<'tcx> {
    fn try_fold_with<F: FallibleTypeFolder<TyCtxt<'tcx>>>(
        self,
        folder: &mut F,
    ) -> Result<Self, F::Error> {
        folder.try_fold_region(self)
    }

    fn fold_with<F: TypeFolder<TyCtxt<'tcx>>>(self, folder: &mut F) -> Self {
        folder.fold_region(self)
    }
}
```

## Block 39
**Metadata**: AST_ID=39 | TYPE=FUNCTION | NAME=visit_with | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx> TypeVisitable<TyCtxt<'tcx>> for ty::Region<'tcx> {
    fn visit_with<V: TypeVisitor<TyCtxt<'tcx>>>(&self, visitor: &mut V) -> V::Result {
        visitor.visit_region(*self)
    }
}
```

## Block 40
**Metadata**: AST_ID=40 | TYPE=FUNCTION | NAME=try_fold_with | COMPLEXITY=6 | LINES=13

```rust
impl<'tcx> TypeFoldable<TyCtxt<'tcx>> for ty::Predicate<'tcx> {
    fn try_fold_with<F: FallibleTypeFolder<TyCtxt<'tcx>>>(
        self,
        folder: &mut F,
    ) -> Result<Self, F::Error> {
        folder.try_fold_predicate(self)
    }

    fn fold_with<F: TypeFolder<TyCtxt<'tcx>>>(self, folder: &mut F) -> Self {
        folder.fold_predicate(self)
    }
}
```

## Block 41
**Metadata**: AST_ID=41 | TYPE=FUNCTION | NAME=try_fold_with | COMPLEXITY=6 | LINES=14

```rust
// FIXME(clause): This is wonky
impl<'tcx> TypeFoldable<TyCtxt<'tcx>> for ty::Clause<'tcx> {
    fn try_fold_with<F: FallibleTypeFolder<TyCtxt<'tcx>>>(
        self,
        folder: &mut F,
    ) -> Result<Self, F::Error> {
        Ok(folder.try_fold_predicate(self.as_predicate())?.expect_clause())
    }

    fn fold_with<F: TypeFolder<TyCtxt<'tcx>>>(self, folder: &mut F) -> Self {
        folder.fold_predicate(self.as_predicate()).expect_clause()
    }
}
```

## Block 42
**Metadata**: AST_ID=42 | TYPE=FUNCTION | NAME=try_fold_with | COMPLEXITY=6 | LINES=13

```rust
impl<'tcx> TypeFoldable<TyCtxt<'tcx>> for ty::Clauses<'tcx> {
    fn try_fold_with<F: FallibleTypeFolder<TyCtxt<'tcx>>>(
        self,
        folder: &mut F,
    ) -> Result<Self, F::Error> {
        folder.try_fold_clauses(self)
    }

    fn fold_with<F: TypeFolder<TyCtxt<'tcx>>>(self, folder: &mut F) -> Self {
        folder.fold_clauses(self)
    }
}
```

## Block 43
**Metadata**: AST_ID=43 | TYPE=FUNCTION | NAME=visit_with | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx> TypeVisitable<TyCtxt<'tcx>> for ty::Predicate<'tcx> {
    fn visit_with<V: TypeVisitor<TyCtxt<'tcx>>>(&self, visitor: &mut V) -> V::Result {
        visitor.visit_predicate(*self)
    }
}
```

## Block 44
**Metadata**: AST_ID=44 | TYPE=FUNCTION | NAME=visit_with | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx> TypeVisitable<TyCtxt<'tcx>> for ty::Clause<'tcx> {
    fn visit_with<V: TypeVisitor<TyCtxt<'tcx>>>(&self, visitor: &mut V) -> V::Result {
        visitor.visit_predicate(self.as_predicate())
    }
}
```

## Block 45
**Metadata**: AST_ID=45 | TYPE=FUNCTION | NAME=try_super_fold_with | COMPLEXITY=7 | LINES=15

```rust
impl<'tcx> TypeSuperFoldable<TyCtxt<'tcx>> for ty::Predicate<'tcx> {
    fn try_super_fold_with<F: FallibleTypeFolder<TyCtxt<'tcx>>>(
        self,
        folder: &mut F,
    ) -> Result<Self, F::Error> {
        let new = self.kind().try_fold_with(folder)?;
        Ok(folder.cx().reuse_or_mk_predicate(self, new))
    }

    fn super_fold_with<F: TypeFolder<TyCtxt<'tcx>>>(self, folder: &mut F) -> Self {
        let new = self.kind().fold_with(folder);
        folder.cx().reuse_or_mk_predicate(self, new)
    }
}
```

## Block 46
**Metadata**: AST_ID=46 | TYPE=FUNCTION | NAME=super_visit_with | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx> TypeSuperVisitable<TyCtxt<'tcx>> for ty::Predicate<'tcx> {
    fn super_visit_with<V: TypeVisitor<TyCtxt<'tcx>>>(&self, visitor: &mut V) -> V::Result {
        self.kind().visit_with(visitor)
    }
}
```

## Block 47
**Metadata**: AST_ID=47 | TYPE=FUNCTION | NAME=visit_with | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx> TypeVisitable<TyCtxt<'tcx>> for ty::Clauses<'tcx> {
    fn visit_with<V: TypeVisitor<TyCtxt<'tcx>>>(&self, visitor: &mut V) -> V::Result {
        visitor.visit_clauses(self)
    }
}
```

## Block 48
**Metadata**: AST_ID=48 | TYPE=FUNCTION | NAME=super_visit_with | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx> TypeSuperVisitable<TyCtxt<'tcx>> for ty::Clauses<'tcx> {
    fn super_visit_with<V: TypeVisitor<TyCtxt<'tcx>>>(&self, visitor: &mut V) -> V::Result {
        self.as_slice().visit_with(visitor)
    }
}
```

## Block 49
**Metadata**: AST_ID=49 | TYPE=FUNCTION | NAME=try_super_fold_with | COMPLEXITY=6 | LINES=13

```rust
impl<'tcx> TypeSuperFoldable<TyCtxt<'tcx>> for ty::Clauses<'tcx> {
    fn try_super_fold_with<F: FallibleTypeFolder<TyCtxt<'tcx>>>(
        self,
        folder: &mut F,
    ) -> Result<Self, F::Error> {
        ty::util::try_fold_list(self, folder, |tcx, v| tcx.mk_clauses(v))
    }

    fn super_fold_with<F: TypeFolder<TyCtxt<'tcx>>>(self, folder: &mut F) -> Self {
        ty::util::fold_list(self, folder, |tcx, v| tcx.mk_clauses(v))
    }
}
```

## Block 50
**Metadata**: AST_ID=50 | TYPE=FUNCTION | NAME=try_fold_with | COMPLEXITY=6 | LINES=13

```rust
impl<'tcx> TypeFoldable<TyCtxt<'tcx>> for ty::Const<'tcx> {
    fn try_fold_with<F: FallibleTypeFolder<TyCtxt<'tcx>>>(
        self,
        folder: &mut F,
    ) -> Result<Self, F::Error> {
        folder.try_fold_const(self)
    }

    fn fold_with<F: TypeFolder<TyCtxt<'tcx>>>(self, folder: &mut F) -> Self {
        folder.fold_const(self)
    }
}
```

## Block 51
**Metadata**: AST_ID=51 | TYPE=FUNCTION | NAME=visit_with | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx> TypeVisitable<TyCtxt<'tcx>> for ty::Const<'tcx> {
    fn visit_with<V: TypeVisitor<TyCtxt<'tcx>>>(&self, visitor: &mut V) -> V::Result {
        visitor.visit_const(*self)
    }
}
```

## Block 52
**Metadata**: AST_ID=52 | TYPE=FUNCTION | NAME=try_super_fold_with | COMPLEXITY=24 | LINES=35

```rust
impl<'tcx> TypeSuperFoldable<TyCtxt<'tcx>> for ty::Const<'tcx> {
    fn try_super_fold_with<F: FallibleTypeFolder<TyCtxt<'tcx>>>(
        self,
        folder: &mut F,
    ) -> Result<Self, F::Error> {
        let kind = match self.kind() {
            ConstKind::Unevaluated(uv) => ConstKind::Unevaluated(uv.try_fold_with(folder)?),
            ConstKind::Value(v) => ConstKind::Value(v.try_fold_with(folder)?),
            ConstKind::Expr(e) => ConstKind::Expr(e.try_fold_with(folder)?),

            ConstKind::Param(_)
            | ConstKind::Infer(_)
            | ConstKind::Bound(..)
            | ConstKind::Placeholder(_)
            | ConstKind::Error(_) => return Ok(self),
        };
        if kind != self.kind() { Ok(folder.cx().mk_ct_from_kind(kind)) } else { Ok(self) }
    }

    fn super_fold_with<F: TypeFolder<TyCtxt<'tcx>>>(self, folder: &mut F) -> Self {
        let kind = match self.kind() {
            ConstKind::Unevaluated(uv) => ConstKind::Unevaluated(uv.fold_with(folder)),
            ConstKind::Value(v) => ConstKind::Value(v.fold_with(folder)),
            ConstKind::Expr(e) => ConstKind::Expr(e.fold_with(folder)),

            ConstKind::Param(_)
            | ConstKind::Infer(_)
            | ConstKind::Bound(..)
            | ConstKind::Placeholder(_)
            | ConstKind::Error(_) => return self,
        };
        if kind != self.kind() { folder.cx().mk_ct_from_kind(kind) } else { self }
    }
}
```

## Block 53
**Metadata**: AST_ID=53 | TYPE=FUNCTION | NAME=super_visit_with | COMPLEXITY=10 | LINES=16

```rust
impl<'tcx> TypeSuperVisitable<TyCtxt<'tcx>> for ty::Const<'tcx> {
    fn super_visit_with<V: TypeVisitor<TyCtxt<'tcx>>>(&self, visitor: &mut V) -> V::Result {
        match self.kind() {
            ConstKind::Unevaluated(uv) => uv.visit_with(visitor),
            ConstKind::Value(v) => v.visit_with(visitor),
            ConstKind::Expr(e) => e.visit_with(visitor),
            ConstKind::Error(e) => e.visit_with(visitor),

            ConstKind::Param(_)
            | ConstKind::Infer(_)
            | ConstKind::Bound(..)
            | ConstKind::Placeholder(_) => V::Result::output(),
        }
    }
}
```

## Block 54
**Metadata**: AST_ID=54 | TYPE=FUNCTION | NAME=visit_with | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx> TypeVisitable<TyCtxt<'tcx>> for crate::rustc_span::ErrorGuaranteed {
    fn visit_with<V: TypeVisitor<TyCtxt<'tcx>>>(&self, visitor: &mut V) -> V::Result {
        visitor.visit_error(*self)
    }
}
```

## Block 55
**Metadata**: AST_ID=55 | TYPE=FUNCTION | NAME=try_fold_with | COMPLEXITY=6 | LINES=13

```rust
impl<'tcx> TypeFoldable<TyCtxt<'tcx>> for crate::rustc_span::ErrorGuaranteed {
    fn try_fold_with<F: FallibleTypeFolder<TyCtxt<'tcx>>>(
        self,
        _folder: &mut F,
    ) -> Result<Self, F::Error> {
        Ok(self)
    }

    fn fold_with<F: TypeFolder<TyCtxt<'tcx>>>(self, _folder: &mut F) -> Self {
        self
    }
}
```

## Block 56
**Metadata**: AST_ID=56 | TYPE=FUNCTION | NAME=visit_with | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx> TypeVisitable<TyCtxt<'tcx>> for TyAndLayout<'tcx, Ty<'tcx>> {
    fn visit_with<V: TypeVisitor<TyCtxt<'tcx>>>(&self, visitor: &mut V) -> V::Result {
        visitor.visit_ty(self.ty)
    }
}
```

## Block 57
**Metadata**: AST_ID=57 | TYPE=FUNCTION | NAME=visit_with | COMPLEXITY=5 | LINES=9

```rust
impl<'tcx, T: TypeVisitable<TyCtxt<'tcx>> + Debug + Clone> TypeVisitable<TyCtxt<'tcx>>
    for Spanned<T>
{
    fn visit_with<V: TypeVisitor<TyCtxt<'tcx>>>(&self, visitor: &mut V) -> V::Result {
        try_visit!(self.node.visit_with(visitor));
        self.span.visit_with(visitor)
    }
}
```

## Block 58
**Metadata**: AST_ID=58 | TYPE=FUNCTION | NAME=try_fold_with | COMPLEXITY=9 | LINES=18

```rust
impl<'tcx, T: TypeFoldable<TyCtxt<'tcx>> + Debug + Clone> TypeFoldable<TyCtxt<'tcx>>
    for Spanned<T>
{
    fn try_fold_with<F: FallibleTypeFolder<TyCtxt<'tcx>>>(
        self,
        folder: &mut F,
    ) -> Result<Self, F::Error> {
        Ok(Spanned {
            node: self.node.try_fold_with(folder)?,
            span: self.span.try_fold_with(folder)?,
        })
    }

    fn fold_with<F: TypeFolder<TyCtxt<'tcx>>>(self, folder: &mut F) -> Self {
        Spanned { node: self.node.fold_with(folder), span: self.span.fold_with(folder) }
    }
}
```

## Block 59
**Metadata**: AST_ID=59 | TYPE=FUNCTION | NAME=try_fold_with | COMPLEXITY=6 | LINES=13

```rust
impl<'tcx> TypeFoldable<TyCtxt<'tcx>> for &'tcx ty::List<LocalDefId> {
    fn try_fold_with<F: FallibleTypeFolder<TyCtxt<'tcx>>>(
        self,
        _folder: &mut F,
    ) -> Result<Self, F::Error> {
        Ok(self)
    }

    fn fold_with<F: TypeFolder<TyCtxt<'tcx>>>(self, _folder: &mut F) -> Self {
        self
    }
}
```

## Block 60
**Metadata**: AST_ID=60 | TYPE=FUNCTION | NAME=try_fold_with | COMPLEXITY=14 | LINES=22

```rust
macro_rules! list_fold {
    ($($ty:ty : $mk:ident),+ $(,)?) => {
        $(
            impl<'tcx> TypeFoldable<TyCtxt<'tcx>> for $ty {
                fn try_fold_with<F: FallibleTypeFolder<TyCtxt<'tcx>>>(
                    self,
                    folder: &mut F,
                ) -> Result<Self, F::Error> {
                    ty::util::try_fold_list(self, folder, |tcx, v| tcx.$mk(v))
                }

                fn fold_with<F: TypeFolder<TyCtxt<'tcx>>>(
                    self,
                    folder: &mut F,
                ) -> Self {
                    ty::util::fold_list(self, folder, |tcx, v| tcx.$mk(v))
                }
            }
        )*
    }
}
```

## Block 61
**Metadata**: AST_ID=61 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
list_fold! {
    &'tcx ty::List<ty::PolyExistentialPredicate<'tcx>> : mk_poly_existential_predicates,
    &'tcx ty::List<PlaceElem<'tcx>> : mk_place_elems,
    &'tcx ty::List<ty::Pattern<'tcx>> : mk_patterns,
    &'tcx ty::List<ty::ArgOutlivesPredicate<'tcx>> : mk_outlives,
}
```

---
*Generated by AST tracing system*
