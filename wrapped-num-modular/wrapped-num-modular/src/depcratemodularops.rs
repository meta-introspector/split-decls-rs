// Generated macro for ModularOps (trait)
macro_rules! DepcrateModularOps {
() => {
// Module: crate
// Provides: {"ModularOps"}
// Dependencies: {}
# [doc = " Collection of common modular arithmetic operations"] pub trait ModularOps < Rhs = Self , Modulus = Self , Output = Self > : ModularCoreOps < Rhs , Modulus , Output = Output > + ModularUnaryOps < Modulus , Output = Output > + ModularPow < Rhs , Modulus , Output = Output > + ModularSymbols < Modulus > { }
};
}
