// Generated macro for impl_57 (impl)
macro_rules! Depcrateimpl_57 {
() => {
// Module: crate
// Provides: {"impl_57"}
// Dependencies: {}
impl Compressor512 { fn new (block : Block512) -> Self { let cv = init512 (transmute ! (block)) ; Compressor512 { cv } } fn input (& mut self , data : & BBGenericArray < u8 , U64 >) { tf512 (& mut self . cv , data) ; } fn finalize_dirty (& mut self) -> Block512 { of512 (& mut self . cv) ; transmute ! (self . cv) } }
};
}
