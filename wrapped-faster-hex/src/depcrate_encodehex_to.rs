// Generated macro for hex_to (function)
macro_rules! Depcrate_encodehex_to {
() => {
// Module: crate::encode
// Provides: {"hex_to"}
// Dependencies: {}
# [deprecated (since = "0.3.0" , note = "please use `hex_encode` instead")] pub fn hex_to (src : & [u8] , dst : & mut [u8]) -> Result < () , Error > { hex_encode (src , dst) . map (| _ | ()) }
};
}
