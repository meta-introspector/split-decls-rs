// Generated macro for impl_260 (impl)
macro_rules! Depcrate_events_attributesimpl_260 {
() => {
// Module: crate::events::attributes
// Provides: {"impl_260"}
// Dependencies: {}
impl Display for AttrError { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { match self { Self :: ExpectedEq (pos) => write ! (f , r#"position {}: attribute key must be directly followed by `=` or space"# , pos) , Self :: ExpectedValue (pos) => write ! (f , r#"position {}: `=` must be followed by an attribute value"# , pos) , Self :: UnquotedValue (pos) => write ! (f , r#"position {}: attribute value must be enclosed in `"` or `'`"# , pos) , Self :: ExpectedQuote (pos , quote) => write ! (f , r#"position {}: missing closing quote `{}` in attribute value"# , pos , * quote as char) , Self :: Duplicated (pos1 , pos2) => write ! (f , r#"position {}: duplicated attribute, previous declaration at position {}"# , pos1 , pos2) , } } }
};
}
