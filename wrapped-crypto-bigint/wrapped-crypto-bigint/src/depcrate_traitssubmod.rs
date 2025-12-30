// Generated macro for SubMod (trait)
macro_rules! Depcrate_traitsSubMod {
() => {
// Module: crate::traits
// Provides: {"SubMod"}
// Dependencies: {}
# [doc = " Compute `self - rhs mod p`."] pub trait SubMod < Rhs = Self , Mod = NonZero < Self > > { # [doc = " Output type."] type Output ; # [doc = " Compute `self - rhs mod p`."] # [doc = ""] # [doc = " Assumes `self` and `rhs` are `< p`."] fn sub_mod (& self , rhs : & Rhs , p : & Mod) -> Self :: Output ; }
};
}
