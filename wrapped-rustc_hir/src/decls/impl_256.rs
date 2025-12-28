macro_rules! deps {
    () => {
        Term!();
        Ty!();
        AssocItemConstraintKind!();
        AssocItemConstraint!();
        ConstArg!();
    };
}

macro_rules! impl_256 {
    () => {
        deps!();
        impl < 'hir > AssocItemConstraint < 'hir > { # [doc = " Obtain the type on the RHS of an assoc ty equality constraint if applicable."] pub fn ty (self) -> Option < & 'hir Ty < 'hir > > { match self . kind { AssocItemConstraintKind :: Equality { term : Term :: Ty (ty) } => Some (ty) , _ => None , } } # [doc = " Obtain the const on the RHS of an assoc const equality constraint if applicable."] pub fn ct (self) -> Option < & 'hir ConstArg < 'hir > > { match self . kind { AssocItemConstraintKind :: Equality { term : Term :: Const (ct) } => Some (ct) , _ => None , } } }
    };
}

impl_256!()