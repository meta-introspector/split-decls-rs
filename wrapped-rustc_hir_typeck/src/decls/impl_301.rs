macro_rules! deps {
    () => {
        BinOpCategory!();
    };
}

macro_rules! impl_301 {
    () => {
        deps!();
        impl From < hir :: BinOpKind > for BinOpCategory { fn from (op : hir :: BinOpKind) -> BinOpCategory { use hir :: BinOpKind :: * ; match op { Shl | Shr => BinOpCategory :: Shift , Add | Sub | Mul | Div | Rem => BinOpCategory :: Math , BitXor | BitAnd | BitOr => BinOpCategory :: Bitwise , Eq | Ne | Lt | Le | Ge | Gt => BinOpCategory :: Comparison , And | Or => BinOpCategory :: Shortcircuit , } } }
    };
}

impl_301!()