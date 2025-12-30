// Generated macro for default_call (function)
macro_rules! Depcrate_deriving_defaultdefault_call {
() => {
// Module: crate::deriving::default
// Provides: {"default_call"}
// Dependencies: {}
fn default_call (cx : & ExtCtxt < '_ > , span : Span) -> Box < ast :: Expr > { let default_ident = cx . std_path (& [kw :: Default , sym :: Default , kw :: Default]) ; cx . expr_call_global (span , default_ident , ThinVec :: new ()) }
};
}
