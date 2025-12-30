// Generated macro for impl_106 (impl)
macro_rules! Depcrate_semanticsimpl_106 {
() => {
// Module: crate::semantics
// Provides: {"impl_106"}
// Dependencies: {}
impl PathResolutionPerNs { pub fn new (type_ns : Option < PathResolution > , value_ns : Option < PathResolution > , macro_ns : Option < PathResolution > ,) -> Self { PathResolutionPerNs { type_ns , value_ns , macro_ns } } pub fn any (& self) -> Option < PathResolution > { self . type_ns . or (self . value_ns) . or (self . macro_ns) } }
};
}
