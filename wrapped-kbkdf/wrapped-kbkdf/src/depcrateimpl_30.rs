// Generated macro for impl_30 (impl)
macro_rules! Depcrateimpl_30 {
() => {
// Module: crate
// Provides: {"impl_30"}
// Dependencies: {}
impl < Prf , K , R > Kbkdf < Prf , K , R > for DoublePipeline < Prf , K , R > where Prf : Mac + KeyInit , K : KeySizeUser , K :: KeySize : ArraySize + Mul < U8 > , < K :: KeySize as Mul < U8 > > :: Output : Unsigned , Prf :: OutputSize : ArraySize + Mul < U8 > , < Prf :: OutputSize as Mul < U8 > > :: Output : Unsigned , R : sealed :: R , { const DOUBLE_PIPELINE : bool = true ; }
};
}
