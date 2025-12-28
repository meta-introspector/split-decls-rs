macro_rules! deps {
    () => {
        CombineAttributeParser!();
        AcceptContext!();
        ForceTargetFeatureParser!();
        AllowedTargets!();
        Stage!();
        ConvertFn!();
        ArgParser!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < S : Stage > CombineAttributeParser < S > for ForceTargetFeatureParser { type Item = (Symbol , Span) ; const PATH : & [Symbol] = & [sym :: force_target_feature] ; const CONVERT : ConvertFn < Self :: Item > = | items , span | AttributeKind :: TargetFeature { features : items , attr_span : span , was_forced : true , } ; const TEMPLATE : AttributeTemplate = template ! (List : & ["enable = \"feat1, feat2\""]) ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn) , Allow (Target :: Method (MethodKind :: Inherent)) , Allow (Target :: Method (MethodKind :: Trait { body : true })) , Allow (Target :: Method (MethodKind :: TraitImpl)) ,]) ; fn extend < 'c > (cx : & 'c mut AcceptContext < '_ , '_ , S > , args : & 'c ArgParser < '_ > ,) -> impl IntoIterator < Item = Self :: Item > + 'c { parse_tf_attribute (cx , args) } }
    };
}

impl_50!();