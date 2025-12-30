// Generated macro for impl_122 (impl)
macro_rules! Depcrate_attributes_crate_levelimpl_122 {
() => {
// Module: crate::attributes::crate_level
// Provides: {"impl_122"}
// Dependencies: {}
impl < S : Stage > SingleAttributeParser < S > for CrateNameParser { const PATH : & [Symbol] = & [sym :: crate_name] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepOutermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: WarnButFutureError ; const TEMPLATE : AttributeTemplate = template ! (NameValueStr : "name") ; const TYPE : AttributeType = AttributeType :: CrateLevel ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (ALL_TARGETS) ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { let ArgParser :: NameValue (n) = args else { cx . expected_name_value (cx . attr_span , None) ; return None ; } ; let Some (name) = n . value_as_str () else { cx . expected_string_literal (n . value_span , Some (n . value_as_lit ())) ; return None ; } ; Some (AttributeKind :: CrateName { name , name_span : n . value_span , attr_span : cx . attr_span , style : cx . attr_style , }) } }
};
}
