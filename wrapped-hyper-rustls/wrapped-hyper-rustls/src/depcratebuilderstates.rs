// Generated macro for builderstates (module)
macro_rules! Depcratebuilderstates {
() => {
// Module: crate
// Provides: {"builderstates"}
// Dependencies: {}
# [doc = " The various states of the [`HttpsConnectorBuilder`]"] pub mod builderstates { # [cfg (feature = "http2")] pub use crate :: connector :: builder :: WantsProtocols3 ; pub use crate :: connector :: builder :: { WantsProtocols1 , WantsProtocols2 , WantsSchemes , WantsTlsConfig , } ; }
};
}
