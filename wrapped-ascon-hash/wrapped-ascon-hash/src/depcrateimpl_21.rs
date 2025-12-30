// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl UpdateCore for AsconCore { fn update_blocks (& mut self , blocks : & [Block < Self >]) { for block in blocks { self . state . absorb_block (block . as_ref ()) ; } } }
};
}
