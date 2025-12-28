macro_rules! deps {
    () => {
        Stage!();
        ConvertFn!();
        ArgParser!();
        AllowInternalUnstableParser!();
        AcceptContext!();
        CombineAttributeParser!();
        AllowedTargets!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl < S : Stage > CombineAttributeParser < S > for AllowInternalUnstableParser { const PATH : & [Symbol] = & [sym :: allow_internal_unstable] ; type Item = (Symbol , Span) ; const CONVERT : ConvertFn < Self :: Item > = | items , span | AttributeKind :: AllowInternalUnstable (items , span) ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: MacroDef) , Allow (Target :: Fn) , Warn (Target :: Field) , Warn (Target :: Arm) ,]) ; const TEMPLATE : AttributeTemplate = template ! (Word , List : & ["feat1, feat2, ..."]) ; fn extend < 'c > (cx : & 'c mut AcceptContext < '_ , '_ , S > , args : & 'c ArgParser < '_ > ,) -> impl IntoIterator < Item = Self :: Item > { parse_unstable (cx , args , < Self as CombineAttributeParser < S > > :: PATH [0]) . into_iter () . zip (iter :: repeat (cx . attr_span)) } }
    };
}

impl_2!();