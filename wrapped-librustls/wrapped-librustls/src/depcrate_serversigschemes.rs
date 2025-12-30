// Generated macro for sigschemes (function)
macro_rules! Depcrate_serversigschemes {
() => {
// Module: crate::server
// Provides: {"sigschemes"}
// Dependencies: {}
fn sigschemes (input : & [u16]) -> Vec < SignatureScheme > { input . iter () . copied () . map (Into :: into) . collect () }
};
}
