macro_rules! UfcsCallInfo {
    () => {
        pub (crate) struct UfcsCallInfo < 'db > { pub (crate) call_expr : ast :: CallExpr , pub (crate) function : hir :: Function , pub (crate) qualifier_type : Option < hir :: Type < 'db > > , }
    };
}

UfcsCallInfo!()