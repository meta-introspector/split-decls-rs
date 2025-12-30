// Generated macro for to_result (function)
macro_rules! Depcrate_systo_result {
() => {
// Module: crate::sys
// Provides: {"to_result"}
// Dependencies: {}
# [cfg (not (feature = "direct-syscall"))] fn to_result (ret : c_int) -> io :: Result < c_int > { if ret >= 0 { Ok (ret) } else { Err (io :: Error :: last_os_error ()) } }
};
}
