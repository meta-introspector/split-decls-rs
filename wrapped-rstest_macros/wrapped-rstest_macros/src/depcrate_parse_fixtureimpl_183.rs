// Generated macro for impl_183 (impl)
macro_rules! Depcrate_parse_fixtureimpl_183 {
() => {
// Module: crate::parse::fixture
// Provides: {"impl_183"}
// Dependencies: {}
impl FixtureModifiers { pub (crate) const DEFAULT_RET_ATTR : & 'static str = "default" ; pub (crate) const PARTIAL_RET_ATTR : & 'static str = "partial_" ; pub (crate) fn extract_default_type (& self) -> Option < syn :: ReturnType > { self . extract_type (Self :: DEFAULT_RET_ATTR) } pub (crate) fn extract_partial_type (& self , pos : usize) -> Option < syn :: ReturnType > { self . extract_type (& format ! ("{}{}" , Self :: PARTIAL_RET_ATTR , pos)) } pub (crate) fn set_default_return_type (& mut self , return_type : syn :: Type) { self . inner . attributes . push (Attribute :: Type (format_ident ! ("{}" , Self :: DEFAULT_RET_ATTR) , Box :: new (return_type) ,)) } pub (crate) fn set_partial_return_type (& mut self , id : usize , return_type : syn :: Type) { self . inner . attributes . push (Attribute :: Type (format_ident ! ("{}{}" , Self :: PARTIAL_RET_ATTR , id) , Box :: new (return_type) ,)) } fn extract_type (& self , attr_name : & str) -> Option < syn :: ReturnType > { self . iter () . filter_map (| m | match m { Attribute :: Type (name , t) if name == attr_name => Some (parse_quote ! { -> # t }) , _ => None , }) . next () } }
};
}
