macro_rules! deps {
    () => {
        BinOpCategory!();
    };
}

macro_rules! impl_302 {
    () => {
        deps!();
        impl From < hir :: AssignOpKind > for BinOpCategory { fn from (op : hir :: AssignOpKind) -> BinOpCategory { use hir :: AssignOpKind :: * ; match op { ShlAssign | ShrAssign => BinOpCategory :: Shift , AddAssign | SubAssign | MulAssign | DivAssign | RemAssign => BinOpCategory :: Math , BitXorAssign | BitAndAssign | BitOrAssign => BinOpCategory :: Bitwise , } } }
    };
}

impl_302!()