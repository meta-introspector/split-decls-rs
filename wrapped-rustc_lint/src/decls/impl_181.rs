macro_rules! deps {
    () => {
        EarlyContextAndPass!();
        EarlyCheckNode!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        impl < 'a > EarlyCheckNode < 'a > for (ast :: NodeId , & 'a [ast :: Attribute] , & 'a [Box < ast :: Item >]) { fn id (self) -> ast :: NodeId { self . 0 } fn attrs (self) -> & 'a [ast :: Attribute] { self . 1 } fn check < 'ecx , 'tcx , T : EarlyLintPass > (self , cx : & mut EarlyContextAndPass < 'ecx , 'tcx , T >) { walk_list ! (cx , visit_attribute , self . 1) ; walk_list ! (cx , visit_item , self . 2) ; } }
    };
}

impl_181!()