// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl < T , Rhs , Modulus > ModularOps < Rhs , Modulus > for T where T : ModularCoreOps < Rhs , Modulus , Output = T > + ModularUnaryOps < Modulus , Output = T > + ModularPow < Rhs , Modulus , Output = T > + ModularSymbols < Modulus > { }
};
}
