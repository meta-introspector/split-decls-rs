// Generated macro for impl_174 (impl)
macro_rules! Depcrate_attributes_link_attrsimpl_174 {
() => {
// Module: crate::attributes::link_attrs
// Provides: {"impl_174"}
// Dependencies: {}
impl < S : Stage > SingleAttributeParser < S > for LinkNameParser { const PATH : & [Symbol] = & [sym :: link_name] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepInnermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: WarnButFutureError ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowListWarnRest (& [Allow (Target :: ForeignFn) , Allow (Target :: ForeignStatic) ,]) ; const TEMPLATE : AttributeTemplate = template ! (NameValueStr : "name" , "https://doc.rust-lang.org/reference/items/external-blocks.html#the-link_name-attribute") ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { let Some (nv) = args . name_value () else { cx . expected_name_value (cx . attr_span , None) ; return None ; } ; let Some (name) = nv . value_as_str () else { cx . expected_string_literal (nv . value_span , Some (nv . value_as_lit ())) ; return None ; } ; Some (LinkName { name , span : cx . attr_span }) } }
};
}
