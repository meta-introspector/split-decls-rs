// Generated macro for impl_32 (impl)
macro_rules! Depcrate_attributes_allow_unstableimpl_32 {
() => {
// Module: crate::attributes::allow_unstable
// Provides: {"impl_32"}
// Dependencies: {}
impl < S : Stage > CombineAttributeParser < S > for AllowConstFnUnstableParser { const PATH : & [Symbol] = & [sym :: rustc_allow_const_fn_unstable] ; type Item = Symbol ; const CONVERT : ConvertFn < Self :: Item > = | items , first_span | AttributeKind :: AllowConstFnUnstable (items , first_span) ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn) , Allow (Target :: Method (MethodKind :: Inherent)) , Allow (Target :: Method (MethodKind :: Trait { body : false })) , Allow (Target :: Method (MethodKind :: Trait { body : true })) , Allow (Target :: Method (MethodKind :: TraitImpl)) ,]) ; const TEMPLATE : AttributeTemplate = template ! (Word , List : & ["feat1, feat2, ..."]) ; fn extend < 'c > (cx : & 'c mut AcceptContext < '_ , '_ , S > , args : & 'c ArgParser < '_ > ,) -> impl IntoIterator < Item = Self :: Item > + 'c { parse_unstable (cx , args , < Self as CombineAttributeParser < S > > :: PATH [0]) } }
};
}
