// Generated macro for Log (trait)
macro_rules! DepcrateLog {
() => {
// Module: crate
// Provides: {"Log"}
// Dependencies: {}
# [doc = " A trait encapsulating the operations required of a logger."] pub trait Log : Sync + Send { # [doc = " Determines if a log message with the specified metadata would be"] # [doc = " logged."] # [doc = ""] # [doc = " This is used by the `log_enabled!` macro to allow callers to avoid"] # [doc = " expensive computation of log message arguments if the message would be"] # [doc = " discarded anyway."] # [doc = ""] # [doc = " # For implementors"] # [doc = ""] # [doc = " This method isn't called automatically by the `log!` macros."] # [doc = " It's up to an implementation of the `Log` trait to call `enabled` in its own"] # [doc = " `log` method implementation to guarantee that filtering is applied."] fn enabled (& self , metadata : & Metadata) -> bool ; # [doc = " Logs the `Record`."] # [doc = ""] # [doc = " # For implementors"] # [doc = ""] # [doc = " Note that `enabled` is *not* necessarily called before this method."] # [doc = " Implementations of `log` should perform all necessary filtering"] # [doc = " internally."] fn log (& self , record : & Record) ; # [doc = " Flushes any buffered records."] # [doc = ""] # [doc = " # For implementors"] # [doc = ""] # [doc = " This method isn't called automatically by the `log!` macros."] # [doc = " It can be called manually on shut-down to ensure any in-flight records are flushed."] fn flush (& self) ; }
};
}
