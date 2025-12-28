macro_rules! deps {
    () => {
        Expr!();
        Block!();
        ExprKind!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl < 'hir > Block < 'hir > { pub fn innermost_block (& self) -> & Block < 'hir > { let mut block = self ; while let Some (Expr { kind : ExprKind :: Block (inner_block , _) , .. }) = block . expr { block = inner_block ; } block } }
    };
}

impl_179!()