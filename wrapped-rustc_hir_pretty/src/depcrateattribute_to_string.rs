// Generated macro for attribute_to_string (function)
macro_rules! Depcrateattribute_to_string {
() => {
// Module: crate
// Provides: {"attribute_to_string"}
// Dependencies: {}
pub fn attribute_to_string (ann : & dyn PpAnn , attr : & hir :: Attribute) -> String { to_string (ann , | s | s . print_attribute_as_style (attr , ast :: AttrStyle :: Outer)) }
};
}
