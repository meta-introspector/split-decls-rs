// Generated macro for NegMod (trait)
macro_rules! Depcrate_traitsNegMod {
() => {
// Module: crate::traits
// Provides: {"NegMod"}
// Dependencies: {}
# [doc = " Compute `-self mod p`."] pub trait NegMod < Mod = NonZero < Self > > { # [doc = " Output type."] type Output ; # [doc = " Compute `-self mod p`."] # [must_use] fn neg_mod (& self , p : & Mod) -> Self :: Output ; }
};
}
