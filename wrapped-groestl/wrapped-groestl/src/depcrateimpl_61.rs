// Generated macro for impl_61 (impl)
macro_rules! Depcrateimpl_61 {
() => {
// Module: crate
// Provides: {"impl_61"}
// Dependencies: {}
impl Compressor1024 { fn new (block : Block1024) -> Self { let cv = init1024 (unsafe { CvBytes1024 { block } . cv }) ; Compressor1024 { cv } } fn input (& mut self , data : & BBGenericArray < u8 , U128 >) { tf1024 (& mut self . cv , data) ; } fn finalize_dirty (& mut self) -> Block1024 { of1024 (& mut self . cv) ; unsafe { CvBytes1024 { cv : self . cv } . block } } }
};
}
