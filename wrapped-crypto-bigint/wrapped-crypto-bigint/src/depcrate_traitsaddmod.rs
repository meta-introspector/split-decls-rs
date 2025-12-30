// Generated macro for AddMod (trait)
macro_rules! Depcrate_traitsAddMod {
() => {
// Module: crate::traits
// Provides: {"AddMod"}
// Dependencies: {}
# [doc = " Compute `self + rhs mod p`."] pub trait AddMod < Rhs = Self , Mod = NonZero < Self > > { # [doc = " Output type."] type Output ; # [doc = " Compute `self + rhs mod p`."] # [doc = ""] # [doc = " Assumes `self` and `rhs` are `< p`."] fn add_mod (& self , rhs : & Rhs , p : & Mod) -> Self :: Output ; }
};
}
