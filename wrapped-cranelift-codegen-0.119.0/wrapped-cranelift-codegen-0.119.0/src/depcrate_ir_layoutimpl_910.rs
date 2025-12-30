// Generated macro for impl_910 (impl)
macro_rules! Depcrate_ir_layoutimpl_910 {
() => {
// Module: crate::ir::layout
// Provides: {"impl_910"}
// Dependencies: {}
impl Layout { # [doc = " Create a new empty `Layout`."] pub fn new () -> Self { Self { blocks : SecondaryMap :: new () , insts : SecondaryMap :: new () , first_block : None , last_block : None , } } # [doc = " Clear the layout."] pub fn clear (& mut self) { self . blocks . clear () ; self . insts . clear () ; self . first_block = None ; self . last_block = None ; } # [doc = " Returns the capacity of the `BlockData` map."] pub fn block_capacity (& self) -> usize { self . blocks . capacity () } }
};
}
