// Generated macro for impl_521 (impl)
macro_rules! Depcrate_displayimpl_521 {
() => {
// Module: crate::display
// Provides: {"impl_521"}
// Dependencies: {}
impl DisplayKind { fn is_source_code (self) -> bool { matches ! (self , Self :: SourceCode { .. }) } fn allows_opaque (self) -> bool { match self { Self :: SourceCode { allow_opaque , .. } => allow_opaque , _ => true , } } }
};
}
