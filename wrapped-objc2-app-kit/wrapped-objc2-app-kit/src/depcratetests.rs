// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [test] # [cfg (feature = "NSAccessibilityProtocols")] fn accessibility_element_protocol () { use crate :: NSAccessibilityElementProtocol ; use objc2 :: ProtocolType ; let actual = < dyn NSAccessibilityElementProtocol > :: NAME ; assert_eq ! (actual , "NSAccessibilityElement") ; } }
};
}
