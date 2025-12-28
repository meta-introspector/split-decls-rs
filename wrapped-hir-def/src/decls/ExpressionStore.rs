macro_rules! deps {
    () => {
        ExpressionOnlyStore!();
    };
}

macro_rules! ExpressionStore {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq)] pub struct ExpressionStore { expr_only : Option < Box < ExpressionOnlyStore > > , pub types : Arena < TypeRef > , pub lifetimes : Arena < LifetimeRef > , }
    };
}

ExpressionStore!();