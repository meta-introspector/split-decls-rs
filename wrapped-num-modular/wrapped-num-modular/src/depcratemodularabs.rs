// Generated macro for ModularAbs (trait)
macro_rules! DepcrateModularAbs {
() => {
// Module: crate
// Provides: {"ModularAbs"}
// Dependencies: {}
# [doc = " Provides a utility function to convert signed integers into unsigned modular form"] pub trait ModularAbs < Modulus > { # [doc = " Return self % m, but accepting signed integers"] fn absm (self , m : & Modulus) -> Modulus ; }
};
}
