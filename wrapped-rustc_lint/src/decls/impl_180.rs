macro_rules! deps {
    () => {
        EarlyCheckNode!();
        EarlyContextAndPass!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl < 'a > EarlyCheckNode < 'a > for (& 'a ast :: Crate , & 'a [ast :: Attribute]) { fn id (self) -> ast :: NodeId { ast :: CRATE_NODE_ID } fn attrs (self) -> & 'a [ast :: Attribute] { self . 1 } fn check < 'ecx , 'tcx , T : EarlyLintPass > (self , cx : & mut EarlyContextAndPass < 'ecx , 'tcx , T >) { lint_callback ! (cx , check_crate , self . 0) ; ast_visit :: walk_crate (cx , self . 0) ; lint_callback ! (cx , check_crate_post , self . 0) ; } }
    };
}

impl_180!();