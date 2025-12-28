macro_rules! deps {
    () => {
        AssignOpKind!();
        BinOpKind!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl From < AssignOpKind > for BinOpKind { fn from (op : AssignOpKind) -> BinOpKind { match op { AssignOpKind :: AddAssign => BinOpKind :: Add , AssignOpKind :: SubAssign => BinOpKind :: Sub , AssignOpKind :: MulAssign => BinOpKind :: Mul , AssignOpKind :: DivAssign => BinOpKind :: Div , AssignOpKind :: RemAssign => BinOpKind :: Rem , AssignOpKind :: BitXorAssign => BinOpKind :: BitXor , AssignOpKind :: BitAndAssign => BinOpKind :: BitAnd , AssignOpKind :: BitOrAssign => BinOpKind :: BitOr , AssignOpKind :: ShlAssign => BinOpKind :: Shl , AssignOpKind :: ShrAssign => BinOpKind :: Shr , } } }
    };
}

impl_66!();