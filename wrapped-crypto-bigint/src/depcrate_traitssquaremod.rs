// Generated macro for SquareMod (trait)
macro_rules! Depcrate_traitsSquareMod {
() => {
// Module: crate::traits
// Provides: {"SquareMod"}
// Dependencies: {}
# [doc = " Compute `self * self mod p`."] pub trait SquareMod < Mod = NonZero < Self > > { # [doc = " Output type."] type Output ; # [doc = " Compute `self * self mod p`."] fn square_mod (& self , p : & Mod) -> Self :: Output ; }
};
}
