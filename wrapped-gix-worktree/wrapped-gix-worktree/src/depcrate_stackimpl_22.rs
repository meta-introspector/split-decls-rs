// Generated macro for impl_22 (impl)
macro_rules! Depcrate_stackimpl_22 {
() => {
// Module: crate::stack
// Provides: {"impl_22"}
// Dependencies: {}
# [doc = " Access"] impl Stack { # [doc = " Return the statistics we gathered thus far."] pub fn statistics (& self) -> & Statistics { & self . statistics } # [doc = " Return the state for introspection."] pub fn state (& self) -> & State { & self . state } # [doc = " Return the base path against which all entries or paths should be relative to when querying."] # [doc = ""] # [doc = " Note that this path _may_ not be canonicalized."] pub fn base (& self) -> & Path { self . stack . root () } }
};
}
