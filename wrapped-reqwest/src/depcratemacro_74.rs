// Generated macro for macro_74 (macro)
macro_rules! Depcratemacro_74 {
() => {
// Module: crate
// Provides: {"macro_74"}
// Dependencies: {}
if_wasm ! { mod wasm ; mod util ; pub use self :: wasm :: { Body , Client , ClientBuilder , Request , RequestBuilder , Response } ; # [cfg (feature = "multipart")] pub use self :: wasm :: multipart ; }
};
}
