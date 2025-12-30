// Generated macro for ModularCoreOps (trait)
macro_rules! DepcrateModularCoreOps {
() => {
// Module: crate
// Provides: {"ModularCoreOps"}
// Dependencies: {}
# [doc = " Core modular arithmetic operations."] # [doc = ""] # [doc = " Note that all functions will panic if the modulus is zero."] pub trait ModularCoreOps < Rhs = Self , Modulus = Self > { type Output ; # [doc = " Return (self + rhs) % m"] fn addm (self , rhs : Rhs , m : Modulus) -> Self :: Output ; # [doc = " Return (self - rhs) % m"] fn subm (self , rhs : Rhs , m : Modulus) -> Self :: Output ; # [doc = " Return (self * rhs) % m"] fn mulm (self , rhs : Rhs , m : Modulus) -> Self :: Output ; }
};
}
