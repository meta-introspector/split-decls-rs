// Generated macro for macro_1 (macro)
macro_rules! Depcratemacro_1 {
() => {
// Module: crate
// Provides: {"macro_1"}
// Dependencies: {}
cfg_if :: cfg_if ! { if # [cfg (any (all (target_arch = "wasm32" , not (target_os = "wasi")) , target_arch = "asmjs"))] { # [cfg (all (feature = "stdweb" , not (feature = "wasm-bindgen")))] # [macro_use] extern crate stdweb ; mod wasm ; pub use wasm :: Instant ; pub use crate :: wasm :: now ; pub use wasm :: SystemTime ; } else { mod native ; pub use native :: Instant ; pub use native :: now ; pub use native :: SystemTime ; } }
};
}
