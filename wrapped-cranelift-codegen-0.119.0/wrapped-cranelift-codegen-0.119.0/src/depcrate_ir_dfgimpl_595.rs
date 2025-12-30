// Generated macro for impl_595 (impl)
macro_rules! Depcrate_ir_dfgimpl_595 {
() => {
// Module: crate::ir::dfg
// Provides: {"impl_595"}
// Dependencies: {}
impl Blocks { # [doc = " Create a new basic block."] pub fn add (& mut self) -> Block { self . 0 . push (BlockData :: new ()) } # [doc = " Get the total number of basic blocks created in this function, whether they are"] # [doc = " currently inserted in the layout or not."] # [doc = ""] # [doc = " This is intended for use with `SecondaryMap::with_capacity`."] pub fn len (& self) -> usize { self . 0 . len () } # [doc = " Returns `true` if the given block reference is valid."] pub fn is_valid (& self , block : Block) -> bool { self . 0 . is_valid (block) } }
};
}
