// Generated macro for impl_23 (impl)
macro_rules! Depcrate_block_apiimpl_23 {
() => {
// Module: crate::block_api
// Provides: {"impl_23"}
// Dependencies: {}
impl UpdateCore for KupynaLongVarCore { # [inline] fn update_blocks (& mut self , blocks : & [Block < Self >]) { self . blocks_len += blocks . len () as u64 ; for block in blocks { long :: compress (& mut self . state , block . as_ref ()) ; } } }
};
}
