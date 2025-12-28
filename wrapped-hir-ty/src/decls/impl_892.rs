macro_rules! deps {
    () => {
        BinOp!();
    };
}

macro_rules! impl_892 {
    () => {
        deps!();
        impl From < hir_def :: hir :: ArithOp > for BinOp { fn from (value : hir_def :: hir :: ArithOp) -> Self { match value { hir_def :: hir :: ArithOp :: Add => BinOp :: Add , hir_def :: hir :: ArithOp :: Mul => BinOp :: Mul , hir_def :: hir :: ArithOp :: Sub => BinOp :: Sub , hir_def :: hir :: ArithOp :: Div => BinOp :: Div , hir_def :: hir :: ArithOp :: Rem => BinOp :: Rem , hir_def :: hir :: ArithOp :: Shl => BinOp :: Shl , hir_def :: hir :: ArithOp :: Shr => BinOp :: Shr , hir_def :: hir :: ArithOp :: BitXor => BinOp :: BitXor , hir_def :: hir :: ArithOp :: BitOr => BinOp :: BitOr , hir_def :: hir :: ArithOp :: BitAnd => BinOp :: BitAnd , } } }
    };
}

impl_892!();