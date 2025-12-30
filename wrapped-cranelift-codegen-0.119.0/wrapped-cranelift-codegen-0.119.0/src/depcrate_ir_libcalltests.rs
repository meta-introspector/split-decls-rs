// Generated macro for tests (module)
macro_rules! Depcrate_ir_libcalltests {
() => {
// Module: crate::ir::libcall
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use alloc :: string :: ToString ; # [test] fn display () { assert_eq ! (LibCall :: CeilF32 . to_string () , "CeilF32") ; assert_eq ! (LibCall :: NearestF64 . to_string () , "NearestF64") ; } # [test] fn parsing () { assert_eq ! ("FloorF32" . parse () , Ok (LibCall :: FloorF32)) ; } # [test] fn all_libcalls_to_from_string () { for & libcall in LibCall :: all_libcalls () { assert_eq ! (libcall . to_string () . parse () , Ok (libcall)) ; } } }
};
}
