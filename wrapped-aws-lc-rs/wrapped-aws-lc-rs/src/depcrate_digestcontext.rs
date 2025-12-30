// Generated macro for Context (struct)
macro_rules! Depcrate_digestContext {
() => {
// Module: crate::digest
// Provides: {"Context"}
// Dependencies: {}
# [doc = " A context for multi-step (Init-Update-Finish) digest calculations."] # [derive (Clone)] pub struct Context { # [doc = " The context's algorithm."] pub (crate) algorithm : & 'static Algorithm , digest_ctx : DigestContext , msg_len : u64 , max_input_reached : bool , }
};
}
