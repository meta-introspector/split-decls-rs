// Generated macro for msg (macro)
macro_rules! Depcratemsg {
() => {
// Module: crate
// Provides: {"msg"}
// Dependencies: {}
# [doc = " Print a message to the log."] # [doc = ""] # [doc = " Supports simple strings as well as Rust [format strings][fs]. When passed a"] # [doc = " single expression it will be passed directly to [`sol_log`]. The expression"] # [doc = " must have type `&str`, and is typically used for logging static strings."] # [doc = " When passed something other than an expression, particularly"] # [doc = " a sequence of expressions, the tokens will be passed through the"] # [doc = " [`format!`] macro before being logged with `sol_log`."] # [doc = ""] # [doc = " [fs]: https://doc.rust-lang.org/alloc/fmt/"] # [doc = " [`format!`]: https://doc.rust-lang.org/alloc/fmt/fn.format.html"] # [doc = ""] # [doc = " Note that Rust's formatting machinery is relatively CPU-intensive"] # [doc = " for constrained environments like the Solana VM."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use solana_msg::msg;"] # [doc = ""] # [doc = " // The fast form"] # [doc = " msg!(\"verifying multisig\");"] # [doc = ""] # [doc = " // With formatting"] # [doc = " let err = \"not enough signers\";"] # [doc = " msg!(\"multisig failed: {}\", err);"] # [doc = " ```"] # [cfg (feature = "alloc")] # [macro_export] macro_rules ! msg { ($ msg : expr) => { $ crate :: sol_log ($ msg) } ; ($ ($ arg : tt) *) => ($ crate :: sol_log (&$ crate :: format ! ($ ($ arg) *))) ; }
};
}
