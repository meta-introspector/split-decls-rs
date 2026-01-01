# AST Trace: ../rust/compiler/rustc_public/src/visitor.rs

Generated 21 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
use std::ops::ControlFlow;

use super::ty::{
    Allocation, Binder, ConstDef, ExistentialPredicate, FnSig, GenericArgKind, GenericArgs,
    MirConst, Promoted, Region, RigidTy, TermKind, Ty, UnevaluatedConst,
};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=FUNCTION | NAME=visit_ty | COMPLEXITY=5 | LINES=15

```rust
use crate::Opaque;
use crate::ty::TyConst;

pub trait Visitor: Sized {
    type Break;
    fn visit_ty(&mut self, ty: &Ty) -> ControlFlow<Self::Break> {
        ty.super_visit(self)
    }
    fn visit_const(&mut self, c: &TyConst) -> ControlFlow<Self::Break> {
        c.super_visit(self)
    }
    fn visit_reg(&mut self, reg: &Region) -> ControlFlow<Self::Break> {
        reg.super_visit(self)
    }
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=visit | COMPLEXITY=3 | LINES=7

```rust
pub trait Visitable {
    fn visit<V: Visitor>(&self, visitor: &mut V) -> ControlFlow<V::Break> {
        self.super_visit(visitor)
    }
    fn super_visit<V: Visitor>(&self, visitor: &mut V) -> ControlFlow<V::Break>;
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=visit | COMPLEXITY=12 | LINES=14

```rust
impl Visitable for Ty {
    fn visit<V: Visitor>(&self, visitor: &mut V) -> ControlFlow<V::Break> {
        visitor.visit_ty(self)
    }
    fn super_visit<V: Visitor>(&self, visitor: &mut V) -> ControlFlow<V::Break> {
        match self.kind() {
            super::ty::TyKind::RigidTy(ty) => ty.visit(visitor)?,
            super::ty::TyKind::Alias(_, alias) => alias.args.visit(visitor)?,
            super::ty::TyKind::Param(_) | super::ty::TyKind::Bound(_, _) => {}
        }
        ControlFlow::Continue(())
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=visit | COMPLEXITY=13 | LINES=18

```rust
impl Visitable for TyConst {
    fn visit<V: Visitor>(&self, visitor: &mut V) -> ControlFlow<V::Break> {
        visitor.visit_const(self)
    }
    fn super_visit<V: Visitor>(&self, visitor: &mut V) -> ControlFlow<V::Break> {
        match &self.kind {
            super::ty::TyConstKind::Param(_) | super::ty::TyConstKind::Bound(_, _) => {}
            super::ty::TyConstKind::Unevaluated(_, args) => args.visit(visitor)?,
            super::ty::TyConstKind::Value(ty, alloc) => {
                alloc.visit(visitor)?;
                ty.visit(visitor)?;
            }
            super::ty::TyConstKind::ZSTValue(ty) => ty.visit(visitor)?,
        }
        ControlFlow::Continue(())
    }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=visit | COMPLEXITY=12 | LINES=15

```rust
impl Visitable for MirConst {
    fn visit<V: Visitor>(&self, visitor: &mut V) -> ControlFlow<V::Break> {
        self.super_visit(visitor)
    }
    fn super_visit<V: Visitor>(&self, visitor: &mut V) -> ControlFlow<V::Break> {
        match &self.kind() {
            super::ty::ConstantKind::Ty(ct) => ct.visit(visitor)?,
            super::ty::ConstantKind::Allocated(alloc) => alloc.visit(visitor)?,
            super::ty::ConstantKind::Unevaluated(uv) => uv.visit(visitor)?,
            super::ty::ConstantKind::Param(_) | super::ty::ConstantKind::ZeroSized => {}
        }
        self.ty().visit(visitor)
    }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=super_visit | COMPLEXITY=5 | LINES=6

```rust
impl Visitable for Opaque {
    fn super_visit<V: Visitor>(&self, _visitor: &mut V) -> ControlFlow<V::Break> {
        ControlFlow::Continue(())
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=super_visit | COMPLEXITY=5 | LINES=6

```rust
impl Visitable for Allocation {
    fn super_visit<V: Visitor>(&self, _visitor: &mut V) -> ControlFlow<V::Break> {
        ControlFlow::Continue(())
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=super_visit | COMPLEXITY=6 | LINES=9

```rust
impl Visitable for UnevaluatedConst {
    fn super_visit<V: Visitor>(&self, visitor: &mut V) -> ControlFlow<V::Break> {
        let UnevaluatedConst { def, args, promoted } = self;
        def.visit(visitor)?;
        args.visit(visitor)?;
        promoted.visit(visitor)
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=super_visit | COMPLEXITY=5 | LINES=6

```rust
impl Visitable for ConstDef {
    fn super_visit<V: Visitor>(&self, _visitor: &mut V) -> ControlFlow<V::Break> {
        ControlFlow::Continue(())
    }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=super_visit | COMPLEXITY=9 | LINES=9

```rust
impl<T: Visitable> Visitable for Option<T> {
    fn super_visit<V: Visitor>(&self, visitor: &mut V) -> ControlFlow<V::Break> {
        match self {
            Some(val) => val.visit(visitor),
            None => ControlFlow::Continue(()),
        }
    }
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=super_visit | COMPLEXITY=5 | LINES=6

```rust
impl Visitable for Promoted {
    fn super_visit<V: Visitor>(&self, _visitor: &mut V) -> ControlFlow<V::Break> {
        ControlFlow::Continue(())
    }
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=super_visit | COMPLEXITY=5 | LINES=6

```rust
impl Visitable for GenericArgs {
    fn super_visit<V: Visitor>(&self, visitor: &mut V) -> ControlFlow<V::Break> {
        self.0.visit(visitor)
    }
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=visit | COMPLEXITY=6 | LINES=10

```rust
impl Visitable for Region {
    fn visit<V: Visitor>(&self, visitor: &mut V) -> ControlFlow<V::Break> {
        visitor.visit_reg(self)
    }

    fn super_visit<V: Visitor>(&self, _: &mut V) -> ControlFlow<V::Break> {
        ControlFlow::Continue(())
    }
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=super_visit | COMPLEXITY=9 | LINES=10

```rust
impl Visitable for GenericArgKind {
    fn super_visit<V: Visitor>(&self, visitor: &mut V) -> ControlFlow<V::Break> {
        match self {
            GenericArgKind::Lifetime(lt) => lt.visit(visitor),
            GenericArgKind::Type(t) => t.visit(visitor),
            GenericArgKind::Const(c) => c.visit(visitor),
        }
    }
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=super_visit | COMPLEXITY=14 | LINES=38

```rust
impl Visitable for RigidTy {
    fn super_visit<V: Visitor>(&self, visitor: &mut V) -> ControlFlow<V::Break> {
        match self {
            RigidTy::Bool
            | RigidTy::Char
            | RigidTy::Int(_)
            | RigidTy::Uint(_)
            | RigidTy::Float(_)
            | RigidTy::Never
            | RigidTy::Foreign(_)
            | RigidTy::Str => ControlFlow::Continue(()),
            RigidTy::Array(t, c) => {
                t.visit(visitor)?;
                c.visit(visitor)
            }
            RigidTy::Pat(t, _p) => t.visit(visitor),
            RigidTy::Slice(inner) => inner.visit(visitor),
            RigidTy::RawPtr(ty, _) => ty.visit(visitor),
            RigidTy::Ref(reg, ty, _) => {
                reg.visit(visitor)?;
                ty.visit(visitor)
            }
            RigidTy::Adt(_, args)
            | RigidTy::Closure(_, args)
            | RigidTy::Coroutine(_, args)
            | RigidTy::CoroutineWitness(_, args)
            | RigidTy::CoroutineClosure(_, args)
            | RigidTy::FnDef(_, args) => args.visit(visitor),
            RigidTy::FnPtr(sig) => sig.visit(visitor),
            RigidTy::Dynamic(pred, r, _) => {
                pred.visit(visitor)?;
                r.visit(visitor)
            }
            RigidTy::Tuple(fields) => fields.visit(visitor),
        }
    }
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=super_visit | COMPLEXITY=8 | LINES=9

```rust
impl<T: Visitable> Visitable for Vec<T> {
    fn super_visit<V: Visitor>(&self, visitor: &mut V) -> ControlFlow<V::Break> {
        for arg in self {
            arg.visit(visitor)?;
        }
        ControlFlow::Continue(())
    }
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=super_visit | COMPLEXITY=5 | LINES=6

```rust
impl<T: Visitable> Visitable for Binder<T> {
    fn super_visit<V: Visitor>(&self, visitor: &mut V) -> ControlFlow<V::Break> {
        self.value.visit(visitor)
    }
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=super_visit | COMPLEXITY=10 | LINES=13

```rust
impl Visitable for ExistentialPredicate {
    fn super_visit<V: Visitor>(&self, visitor: &mut V) -> ControlFlow<V::Break> {
        match self {
            ExistentialPredicate::Trait(tr) => tr.generic_args.visit(visitor),
            ExistentialPredicate::Projection(p) => {
                p.term.visit(visitor)?;
                p.generic_args.visit(visitor)
            }
            ExistentialPredicate::AutoTrait(_) => ControlFlow::Continue(()),
        }
    }
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=super_visit | COMPLEXITY=9 | LINES=9

```rust
impl Visitable for TermKind {
    fn super_visit<V: Visitor>(&self, visitor: &mut V) -> ControlFlow<V::Break> {
        match self {
            TermKind::Type(t) => t.visit(visitor),
            TermKind::Const(c) => c.visit(visitor),
        }
    }
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=super_visit | COMPLEXITY=5 | LINES=6

```rust
impl Visitable for FnSig {
    fn super_visit<V: Visitor>(&self, visitor: &mut V) -> ControlFlow<V::Break> {
        self.inputs_and_output.visit(visitor)
    }
}
```

---
*Generated by AST tracing system*
