// Generated macro for MulMod (trait)
macro_rules! Depcrate_traitsMulMod {
() => {
// Module: crate::traits
// Provides: {"MulMod"}
// Dependencies: {}
# [doc = " Compute `self * rhs mod p`."] pub trait MulMod < Rhs = Self , Mod = NonZero < Self > > { # [doc = " Output type."] type Output ; # [doc = " Compute `self * rhs mod p`."] fn mul_mod (& self , rhs : & Rhs , p : & Mod) -> Self :: Output ; }
};
}
