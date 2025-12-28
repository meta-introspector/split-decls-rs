macro_rules! deps {
    () => {
        ResolvedPath!();
        Placeholder!();
        UfcsCallInfo!();
    };
}

macro_rules! ResolvedPattern {
    () => {
        deps!();
        pub (crate) struct ResolvedPattern < 'db > { pub (crate) placeholders_by_stand_in : FxHashMap < SmolStr , parsing :: Placeholder > , pub (crate) node : SyntaxNode , pub (crate) resolved_paths : FxHashMap < SyntaxNode , ResolvedPath > , pub (crate) ufcs_function_calls : FxHashMap < SyntaxNode , UfcsCallInfo < 'db > > , pub (crate) contains_self : bool , }
    };
}

ResolvedPattern!()