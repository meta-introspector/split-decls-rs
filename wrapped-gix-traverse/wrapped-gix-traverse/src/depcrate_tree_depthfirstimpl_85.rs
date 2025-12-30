// Generated macro for impl_85 (impl)
macro_rules! Depcrate_tree_depthfirstimpl_85 {
() => {
// Module: crate::tree::depthfirst
// Provides: {"impl_85"}
// Dependencies: {}
impl State { # [doc = " Pop one empty buffer from the free-list."] pub fn pop_buf (& mut self) -> Vec < u8 > { match self . freelist . pop () { None => Vec :: new () , Some (mut buf) => { buf . clear () ; buf } } } # [doc = " Make `buf` available for re-use with [`Self::pop_buf()`]."] pub fn push_buf (& mut self , buf : Vec < u8 >) { self . freelist . push (buf) ; } }
};
}
