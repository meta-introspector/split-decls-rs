// Generated macro for impl_227 (impl)
macro_rules! Depcrate_replimpl_227 {
() => {
// Module: crate::repl
// Provides: {"impl_227"}
// Dependencies: {}
# [cfg (feature = "async")] impl < S > ReplSession < S > where S : AsyncExpect + Unpin , { # [doc = " Block until prompt is found"] pub async fn expect_prompt (& mut self) -> Result < () , Error > { let _ = self . _expect_prompt () . await ? ; Ok (()) } async fn _expect_prompt (& mut self) -> Result < Captures , Error > { self . session . expect (& self . prompt) . await } }
};
}
