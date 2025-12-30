// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
impl < OutputLen , PrfOutputLen > KbkdfUser for KbkdfCore < OutputLen , PrfOutputLen > where OutputLen : ArraySize + Mul < U8 > , < OutputLen as Mul < U8 > > :: Output : Unsigned , PrfOutputLen : ArraySize + Mul < U8 > , < PrfOutputLen as Mul < U8 > > :: Output : Unsigned , { type L = op ! (OutputLen * U8) ; type H = op ! (PrfOutputLen * U8) ; }
};
}
