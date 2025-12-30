// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
impl UpdateCore for AsconXofCore { fn update_blocks (& mut self , blocks : & [Block < Self >]) { for block in blocks { self . state . absorb_block (block . as_ref ()) ; } } }
};
}
