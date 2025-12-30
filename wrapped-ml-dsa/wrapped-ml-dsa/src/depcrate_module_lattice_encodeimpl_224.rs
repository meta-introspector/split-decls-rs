// Generated macro for impl_224 (impl)
macro_rules! Depcrate_module_lattice_encodeimpl_224 {
() => {
// Module: crate::module_lattice::encode
// Provides: {"impl_224"}
// Dependencies: {}
impl < D > EncodingSize for D where D : ArraySize + Mul < U8 > + Gcd < U8 > + Mul < U32 > , Prod < D , U32 > : ArraySize , Prod < D , U8 > : Div < Gcf < D , U8 > > , EncodingUnit < D > : Div < D > + Div < U8 > , Quot < EncodingUnit < D > , D > : ArraySize , Quot < EncodingUnit < D > , U8 > : ArraySize , { type EncodedPolynomialSize = Prod < D , U32 > ; type ValueStep = Quot < EncodingUnit < D > , D > ; type ByteStep = Quot < EncodingUnit < D > , U8 > ; }
};
}
