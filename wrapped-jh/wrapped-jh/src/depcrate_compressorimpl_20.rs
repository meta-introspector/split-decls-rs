// Generated macro for impl_20 (impl)
macro_rules! Depcrate_compressorimpl_20 {
() => {
// Module: crate::compressor
// Provides: {"impl_20"}
// Dependencies: {}
impl Compressor { # [inline] pub fn new (bytes : [u8 ; 128]) -> Self { Compressor { cv : transmute ! (bytes) , } } # [inline] pub fn input (& mut self , data : & GenericArray < u8 , U64 >) { f8 (& mut self . cv , data . as_ptr ()) } # [inline] pub fn finalize (self) -> [u8 ; 128] { transmute ! (self . cv) } }
};
}
