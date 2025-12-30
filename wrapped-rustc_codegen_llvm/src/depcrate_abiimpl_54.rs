// Generated macro for impl_54 (impl)
macro_rules! Depcrate_abiimpl_54 {
() => {
// Module: crate::abi
// Provides: {"impl_54"}
// Dependencies: {}
impl LlvmType for Reg { fn llvm_type < 'll > (& self , cx : & CodegenCx < 'll , '_ >) -> & 'll Type { match self . kind { RegKind :: Integer => cx . type_ix (self . size . bits ()) , RegKind :: Float => match self . size . bits () { 16 => cx . type_f16 () , 32 => cx . type_f32 () , 64 => cx . type_f64 () , 128 => cx . type_f128 () , _ => bug ! ("unsupported float: {:?}" , self) , } , RegKind :: Vector => cx . type_vector (cx . type_i8 () , self . size . bytes ()) , } } }
};
}
