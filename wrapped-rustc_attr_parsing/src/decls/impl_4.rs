macro_rules! deps {
    () => {
        StabilityOutsideStd!();
        ConvertFn!();
        AllowedTargets!();
        AcceptContext!();
        ArgParser!();
        UnstableFeatureBoundParser!();
        Stage!();
        CombineAttributeParser!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < S : Stage > CombineAttributeParser < S > for UnstableFeatureBoundParser { const PATH : & 'static [rustc_span :: Symbol] = & [sym :: unstable_feature_bound] ; type Item = (Symbol , Span) ; const CONVERT : ConvertFn < Self :: Item > = | items , _ | AttributeKind :: UnstableFeatureBound (items) ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn) , Allow (Target :: Impl { of_trait : true }) , Allow (Target :: Trait) ,]) ; const TEMPLATE : AttributeTemplate = template ! (Word , List : & ["feat1, feat2, ..."]) ; fn extend < 'c > (cx : & 'c mut AcceptContext < '_ , '_ , S > , args : & 'c ArgParser < '_ > ,) -> impl IntoIterator < Item = Self :: Item > { if ! cx . features () . staged_api () { cx . emit_err (session_diagnostics :: StabilityOutsideStd { span : cx . attr_span }) ; } parse_unstable (cx , args , < Self as CombineAttributeParser < S > > :: PATH [0]) . into_iter () . zip (iter :: repeat (cx . attr_span)) } }
    };
}

impl_4!()