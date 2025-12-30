// Generated macro for impl_105 (impl)
macro_rules! Depcrate_attributes_codegen_attrsimpl_105 {
() => {
// Module: crate::attributes::codegen_attrs
// Provides: {"impl_105"}
// Dependencies: {}
impl < S : Stage > CombineAttributeParser < S > for TargetFeatureParser { type Item = (Symbol , Span) ; const PATH : & [Symbol] = & [sym :: target_feature] ; const CONVERT : ConvertFn < Self :: Item > = | items , span | AttributeKind :: TargetFeature { features : items , attr_span : span , was_forced : false , } ; const TEMPLATE : AttributeTemplate = template ! (List : & ["enable = \"feat1, feat2\""]) ; fn extend < 'c > (cx : & 'c mut AcceptContext < '_ , '_ , S > , args : & 'c ArgParser < '_ > ,) -> impl IntoIterator < Item = Self :: Item > + 'c { parse_tf_attribute (cx , args) } const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn) , Allow (Target :: Method (MethodKind :: Inherent)) , Allow (Target :: Method (MethodKind :: Trait { body : true })) , Allow (Target :: Method (MethodKind :: TraitImpl)) , Warn (Target :: Statement) , Warn (Target :: Field) , Warn (Target :: Arm) , Warn (Target :: MacroDef) , Warn (Target :: MacroCall) ,]) ; }
};
}
