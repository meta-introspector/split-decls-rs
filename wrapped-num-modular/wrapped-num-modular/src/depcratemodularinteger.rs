// Generated macro for ModularInteger (trait)
macro_rules! DepcrateModularInteger {
() => {
// Module: crate
// Provides: {"ModularInteger"}
// Dependencies: {}
# [doc = " Represents an number defined in a modulo ring ℤ/nℤ"] # [doc = ""] # [doc = " The operators should panic if the modulus of two number"] # [doc = " are not the same."] pub trait ModularInteger : Sized + PartialEq + Add < Self , Output = Self > + Sub < Self , Output = Self > + Neg < Output = Self > + Mul < Self , Output = Self > { # [doc = " The underlying representation type of the integer"] type Base ; # [doc = " Return the modulus of the ring"] fn modulus (& self) -> Self :: Base ; # [doc = " Return the normalized residue of this integer in the ring"] fn residue (& self) -> Self :: Base ; # [doc = " Check if the integer is zero"] fn is_zero (& self) -> bool ; # [doc = " Convert an normal integer into the same ring."] # [doc = ""] # [doc = " This method should be perferred over the static"] # [doc = " constructor to prevent unnecessary overhead of pre-computation."] fn convert (& self , n : Self :: Base) -> Self ; # [doc = " Calculate the value of self + self"] fn double (self) -> Self ; # [doc = " Calculate the value of self * self"] fn square (self) -> Self ; }
};
}
