// Generated macro for impl_226 (impl)
macro_rules! Depcrate_replimpl_226 {
() => {
// Module: crate::repl
// Provides: {"impl_226"}
// Dependencies: {}
# [cfg (not (feature = "async"))] impl < S > ReplSession < S > where S : Expect , { # [doc = " Block until prompt is found"] pub fn expect_prompt (& mut self) -> Result < () , Error > { let _ = self . _expect_prompt () ? ; Ok (()) } fn _expect_prompt (& mut self) -> Result < Captures , Error > { self . session . expect (& self . prompt) } }
};
}
