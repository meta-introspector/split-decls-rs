// Generated macro for impl_244 (impl)
macro_rules! Depcrate_datetimeimpl_244 {
() => {
// Module: crate::datetime
// Provides: {"impl_244"}
// Dependencies: {}
# [cfg (all (target_arch = "wasm32" , feature = "wasmbind" , not (any (target_os = "emscripten" , target_os = "wasi" , target_os = "linux"))))] impl From < js_sys :: Date > for DateTime < Utc > { fn from (date : js_sys :: Date) -> DateTime < Utc > { DateTime :: < Utc > :: from (& date) } }
};
}
