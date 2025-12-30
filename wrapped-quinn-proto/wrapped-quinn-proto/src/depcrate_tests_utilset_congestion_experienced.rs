// Generated macro for set_congestion_experienced (function)
macro_rules! Depcrate_tests_utilset_congestion_experienced {
() => {
// Module: crate::tests::util
// Provides: {"set_congestion_experienced"}
// Dependencies: {}
fn set_congestion_experienced (x : Option < EcnCodepoint > , congestion_experienced : bool ,) -> Option < EcnCodepoint > { x . map (| codepoint | match congestion_experienced { true => EcnCodepoint :: Ce , false => codepoint , }) }
};
}
