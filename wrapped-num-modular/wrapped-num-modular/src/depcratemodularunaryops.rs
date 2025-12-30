// Generated macro for ModularUnaryOps (trait)
macro_rules! DepcrateModularUnaryOps {
() => {
// Module: crate
// Provides: {"ModularUnaryOps"}
// Dependencies: {}
# [doc = " Core unary modular arithmetics"] # [doc = ""] # [doc = " Note that all functions will panic if the modulus is zero."] pub trait ModularUnaryOps < Modulus = Self > { type Output ; # [doc = " Return (-self) % m and make sure the result is normalized in range [0,m)"] fn negm (self , m : Modulus) -> Self :: Output ; # [doc = " Calculate modular inverse (x such that self*x = 1 mod m)."] # [doc = ""] # [doc = " This operation is only available for integer that is coprime to `m`. If not,"] # [doc = " the result will be [None]."] fn invm (self , m : Modulus) -> Option < Self :: Output > ; # [doc = " Calculate modular double ( x+x mod m)"] fn dblm (self , m : Modulus) -> Self :: Output ; # [doc = " Calculate modular square ( x*x mod m )"] fn sqm (self , m : Modulus) -> Self :: Output ; }
};
}
