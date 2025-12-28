macro_rules! deps {
    () => {
        AssocItemConstraintKind!();
    };
}

macro_rules! impl_261 {
    () => {
        deps!();
        impl < 'hir > AssocItemConstraintKind < 'hir > { pub fn descr (& self) -> & 'static str { match self { AssocItemConstraintKind :: Equality { .. } => "binding" , AssocItemConstraintKind :: Bound { .. } => "constraint" , } } }
    };
}

impl_261!()