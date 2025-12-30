// Generated macro for impl_672 (impl)
macro_rules! Depcrate_vm_init_argsimpl_672 {
() => {
// Module: crate::vm::init_args
// Provides: {"impl_672"}
// Dependencies: {}
impl JvmError { # [doc = " Returns the JVM option that caused the error, if it was caused by one."] pub fn opt_string (& self) -> Option < & str > { match self { Self :: NullOptString (opt_string) => Some (opt_string) , Self :: OptStringTooLong { opt_string , .. } => Some (opt_string) , Self :: OptStringNotRepresentable { opt_string , .. } => Some (opt_string) , Self :: OptStringTranscodeFailure { opt_string , .. } => Some (opt_string) , } . map (String :: as_str) } # [cfg (all (test , windows))] fn opt_string_mut (& mut self) -> Option < & mut String > { match self { Self :: NullOptString (opt_string) => Some (opt_string) , Self :: OptStringTooLong { opt_string , .. } => Some (opt_string) , Self :: OptStringNotRepresentable { opt_string , .. } => Some (opt_string) , Self :: OptStringTranscodeFailure { opt_string , .. } => Some (opt_string) , } } }
};
}
