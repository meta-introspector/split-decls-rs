# AST Trace: ../rust/compiler/rustc_middle/src/ty/consts/kind.rs

Generated 7 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use std::assert_matches::assert_matches;

use rustc_macros::{HashStable, TyDecodable, TyEncodable, TypeFoldable, TypeVisitable};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
use super::Const;
use crate::mir;
use crate::ty::abstract_const::CastKind;
use crate::ty::{self, Ty, TyCtxt};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[derive(HashStable, TyEncodable, TyDecodable, TypeVisitable, TypeFoldable)]
pub enum ExprKind {
    Binop(mir::BinOp),
    UnOp(mir::UnOp),
    FunctionCall,
    Cast(CastKind),
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=STRUCT | NAME=Expr | COMPLEXITY=2 | LINES=6

```rust
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[derive(HashStable, TyEncodable, TyDecodable, TypeVisitable, TypeFoldable)]
pub struct Expr<'tcx> {
    pub kind: ExprKind,
    args: ty::GenericArgsRef<'tcx>,
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=args | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx> rustc_type_ir::inherent::ExprConst<TyCtxt<'tcx>> for Expr<'tcx> {
    fn args(self) -> ty::GenericArgsRef<'tcx> {
        self.args
    }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=new_binop | COMPLEXITY=52 | LINES=108

```rust
impl<'tcx> Expr<'tcx> {
    pub fn new_binop(
        tcx: TyCtxt<'tcx>,
        binop: mir::BinOp,
        lhs_ty: Ty<'tcx>,
        rhs_ty: Ty<'tcx>,
        lhs_ct: Const<'tcx>,
        rhs_ct: Const<'tcx>,
    ) -> Self {
        let args = tcx.mk_args_from_iter::<_, ty::GenericArg<'tcx>>(
            [lhs_ty.into(), rhs_ty.into(), lhs_ct.into(), rhs_ct.into()].into_iter(),
        );

        Self { kind: ExprKind::Binop(binop), args }
    }

    pub fn binop_args(self) -> (Ty<'tcx>, Ty<'tcx>, Const<'tcx>, Const<'tcx>) {
        assert_matches!(self.kind, ExprKind::Binop(_));

        match self.args().as_slice() {
            [lhs_ty, rhs_ty, lhs_ct, rhs_ct] => (
                lhs_ty.expect_ty(),
                rhs_ty.expect_ty(),
                lhs_ct.expect_const(),
                rhs_ct.expect_const(),
            ),
            _ => bug!("Invalid args for `Binop` expr {self:?}"),
        }
    }

    pub fn new_unop(tcx: TyCtxt<'tcx>, unop: mir::UnOp, ty: Ty<'tcx>, ct: Const<'tcx>) -> Self {
        let args =
            tcx.mk_args_from_iter::<_, ty::GenericArg<'tcx>>([ty.into(), ct.into()].into_iter());

        Self { kind: ExprKind::UnOp(unop), args }
    }

    pub fn unop_args(self) -> (Ty<'tcx>, Const<'tcx>) {
        assert_matches!(self.kind, ExprKind::UnOp(_));

        match self.args().as_slice() {
            [ty, ct] => (ty.expect_ty(), ct.expect_const()),
            _ => bug!("Invalid args for `UnOp` expr {self:?}"),
        }
    }

    pub fn new_call(
        tcx: TyCtxt<'tcx>,
        func_ty: Ty<'tcx>,
        func_expr: Const<'tcx>,
        arguments: impl IntoIterator<Item = Const<'tcx>>,
    ) -> Self {
        let args = tcx.mk_args_from_iter::<_, ty::GenericArg<'tcx>>(
            [func_ty.into(), func_expr.into()]
                .into_iter()
                .chain(arguments.into_iter().map(|ct| ct.into())),
        );

        Self { kind: ExprKind::FunctionCall, args }
    }

    pub fn call_args(self) -> (Ty<'tcx>, Const<'tcx>, impl Iterator<Item = Const<'tcx>>) {
        assert_matches!(self.kind, ExprKind::FunctionCall);

        match self.args().as_slice() {
            [func_ty, func, rest @ ..] => (
                func_ty.expect_ty(),
                func.expect_const(),
                rest.iter().map(|arg| arg.expect_const()),
            ),
            _ => bug!("Invalid args for `Call` expr {self:?}"),
        }
    }

    pub fn new_cast(
        tcx: TyCtxt<'tcx>,
        cast: CastKind,
        value_ty: Ty<'tcx>,
        value: Const<'tcx>,
        to_ty: Ty<'tcx>,
    ) -> Self {
        let args = tcx.mk_args_from_iter::<_, ty::GenericArg<'tcx>>(
            [value_ty.into(), value.into(), to_ty.into()].into_iter(),
        );

        Self { kind: ExprKind::Cast(cast), args }
    }

    pub fn cast_args(self) -> (Ty<'tcx>, Const<'tcx>, Ty<'tcx>) {
        assert_matches!(self.kind, ExprKind::Cast(_));

        match self.args().as_slice() {
            [value_ty, value, to_ty] => {
                (value_ty.expect_ty(), value.expect_const(), to_ty.expect_ty())
            }
            _ => bug!("Invalid args for `Cast` expr {self:?}"),
        }
    }

    pub fn new(kind: ExprKind, args: ty::GenericArgsRef<'tcx>) -> Self {
        Self { kind, args }
    }

    pub fn args(self) -> ty::GenericArgsRef<'tcx> {
        self.args
    }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=1 | LINES=3

```rust
#[cfg(target_pointer_width = "64")]
rustc_data_structures::static_assert_size!(Expr<'_>, 16);
```

---
*Generated by AST tracing system*
