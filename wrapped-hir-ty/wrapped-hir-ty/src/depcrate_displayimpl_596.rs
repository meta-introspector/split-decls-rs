// Generated macro for impl_596 (impl)
macro_rules! Depcrate_displayimpl_596 {
() => {
// Module: crate::display
// Provides: {"impl_596"}
// Dependencies: {}
impl DisplayKind { fn is_source_code (self) -> bool { matches ! (self , Self :: SourceCode { .. }) } fn is_test (self) -> bool { matches ! (self , Self :: Test) } fn allows_opaque (self) -> bool { match self { Self :: SourceCode { allow_opaque , .. } => allow_opaque , _ => true , } } }
};
}
