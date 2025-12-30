// Generated macro for ModularPow (trait)
macro_rules! DepcrateModularPow {
() => {
// Module: crate
// Provides: {"ModularPow"}
// Dependencies: {}
# [doc = " Modular power functions"] pub trait ModularPow < Exp = Self , Modulus = Self > { type Output ; # [doc = " Return (self ^ exp) % m"] fn powm (self , exp : Exp , m : Modulus) -> Self :: Output ; }
};
}
