// Generated macro for type_error (function)
macro_rules! Depcrate_rttype_error {
() => {
// Module: crate::rt
// Provides: {"type_error"}
// Dependencies: {}
# [inline] # [cold] fn type_error < A > (msg : & str) -> ! { error ! ("{msg}, expected type: {}" , std :: any :: type_name ::< A > ()) ; std :: panic :: panic_any (Error :: TypeErr) }
};
}
