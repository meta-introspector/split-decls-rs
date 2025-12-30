// Generated macro for impl_126 (impl)
macro_rules! Depcrate_attributes_crate_levelimpl_126 {
() => {
// Module: crate::attributes::crate_level
// Provides: {"impl_126"}
// Dependencies: {}
impl < S : Stage > SingleAttributeParser < S > for MoveSizeLimitParser { const PATH : & [Symbol] = & [sym :: move_size_limit] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepOutermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const TEMPLATE : AttributeTemplate = template ! (NameValueStr : "N") ; const TYPE : AttributeType = AttributeType :: CrateLevel ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (ALL_TARGETS) ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { let ArgParser :: NameValue (nv) = args else { cx . expected_name_value (cx . attr_span , None) ; return None ; } ; Some (AttributeKind :: MoveSizeLimit { limit : cx . parse_limit_int (nv) ? , attr_span : cx . attr_span , limit_span : nv . value_span , }) } }
};
}
