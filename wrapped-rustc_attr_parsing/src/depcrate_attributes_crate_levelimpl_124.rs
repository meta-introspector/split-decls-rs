// Generated macro for impl_124 (impl)
macro_rules! Depcrate_attributes_crate_levelimpl_124 {
() => {
// Module: crate::attributes::crate_level
// Provides: {"impl_124"}
// Dependencies: {}
impl < S : Stage > SingleAttributeParser < S > for RecursionLimitParser { const PATH : & [Symbol] = & [sym :: recursion_limit] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepOutermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: WarnButFutureError ; const TEMPLATE : AttributeTemplate = template ! (NameValueStr : "N" , "https://doc.rust-lang.org/reference/attributes/limits.html#the-recursion_limit-attribute") ; const TYPE : AttributeType = AttributeType :: CrateLevel ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (ALL_TARGETS) ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { let ArgParser :: NameValue (nv) = args else { cx . expected_name_value (cx . attr_span , None) ; return None ; } ; Some (AttributeKind :: RecursionLimit { limit : cx . parse_limit_int (nv) ? , attr_span : cx . attr_span , limit_span : nv . value_span , }) } }
};
}
