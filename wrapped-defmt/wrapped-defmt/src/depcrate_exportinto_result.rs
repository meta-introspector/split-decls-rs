// Generated macro for into_result (function)
macro_rules! Depcrate_exportinto_result {
() => {
// Module: crate::export
// Provides: {"into_result"}
// Dependencies: {}
pub fn into_result < T : traits :: IntoResult > (x : T) -> Result < T :: Ok , T :: Error > { x . into_result () }
};
}
