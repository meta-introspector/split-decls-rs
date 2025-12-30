// Generated macro for impl_40 (impl)
macro_rules! Depcrate_abiimpl_40 {
() => {
// Module: crate::abi
// Provides: {"impl_40"}
// Dependencies: {}
impl GccType for Reg { fn gcc_type < 'gcc > (& self , cx : & CodegenCx < 'gcc , '_ >) -> Type < 'gcc > { match self . kind { RegKind :: Integer => cx . type_ix (self . size . bits ()) , RegKind :: Float => match self . size . bits () { 32 => cx . type_f32 () , 64 => cx . type_f64 () , _ => bug ! ("unsupported float: {:?}" , self) , } , RegKind :: Vector => unimplemented ! () , } } }
};
}
