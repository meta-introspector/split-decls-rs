// Generated macro for impl_11 (impl)
macro_rules! Depcrate_block_apiimpl_11 {
() => {
// Module: crate::block_api
// Provides: {"impl_11"}
// Dependencies: {}
impl UpdateCore for KupynaShortVarCore { # [inline] fn update_blocks (& mut self , blocks : & [Block < Self >]) { self . blocks_len += blocks . len () as u64 ; for block in blocks { short :: compress (& mut self . state , block . as_ref ()) ; } } }
};
}
