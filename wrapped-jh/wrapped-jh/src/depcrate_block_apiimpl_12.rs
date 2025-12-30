// Generated macro for impl_12 (impl)
macro_rules! Depcrate_block_apiimpl_12 {
() => {
// Module: crate::block_api
// Provides: {"impl_12"}
// Dependencies: {}
impl UpdateCore for JhCore { # [inline] fn update_blocks (& mut self , blocks : & [Block < Self >]) { self . block_len = self . block_len . wrapping_add (blocks . len () as u64) ; for b in blocks { self . state . update (b) ; } } }
};
}
