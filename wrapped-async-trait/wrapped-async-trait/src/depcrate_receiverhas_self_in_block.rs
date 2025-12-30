// Generated macro for has_self_in_block (function)
macro_rules! Depcrate_receiverhas_self_in_block {
() => {
// Module: crate::receiver
// Provides: {"has_self_in_block"}
// Dependencies: {}
pub fn has_self_in_block (block : & mut Block) -> bool { let mut visitor = HasSelf (false) ; visitor . visit_block_mut (block) ; visitor . 0 }
};
}
