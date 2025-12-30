// Generated macro for log_message (function)
macro_rules! Depcrate_loggerlog_message {
() => {
// Module: crate::logger
// Provides: {"log_message"}
// Dependencies: {}
# [doc = " Log a message."] # [inline (always)] pub fn log_message (message : & [u8]) { # [cfg (any (target_os = "solana" , target_arch = "bpf"))] unsafe { sol_log_ (message . as_ptr () , message . len () as u64) ; } # [cfg (all (not (any (target_os = "solana" , target_arch = "bpf")) , feature = "std"))] { let message = core :: str :: from_utf8 (message) . unwrap () ; std :: println ! ("{message}") ; } # [cfg (all (not (any (target_os = "solana" , target_arch = "bpf")) , not (feature = "std")))] core :: hint :: black_box (message) ; }
};
}
