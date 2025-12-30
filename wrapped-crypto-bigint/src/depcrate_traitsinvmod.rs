// Generated macro for InvMod (trait)
macro_rules! Depcrate_traitsInvMod {
() => {
// Module: crate::traits
// Provides: {"InvMod"}
// Dependencies: {}
# [doc = " Compute `1 / self mod p`."] # [deprecated (since = "0.7.0" , note = "please use `InvertMod` instead")] pub trait InvMod < Rhs = Self > : Sized { # [doc = " Output type."] type Output ; # [doc = " Compute `1 / self mod p`."] fn inv_mod (& self , p : & Rhs) -> CtOption < Self :: Output > ; }
};
}
