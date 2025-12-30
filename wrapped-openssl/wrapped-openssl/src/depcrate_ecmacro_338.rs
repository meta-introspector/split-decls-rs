// Generated macro for macro_338 (macro)
macro_rules! Depcrate_ecmacro_338 {
() => {
// Module: crate::ec
// Provides: {"macro_338"}
// Dependencies: {}
cfg_if ! { if # [cfg (not (any (boringssl , awslc)))] { use std :: ffi :: CString ; use crate :: string :: OpensslString ; } }
};
}
