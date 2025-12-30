// Generated macro for InvertMod (trait)
macro_rules! Depcrate_traitsInvertMod {
() => {
// Module: crate::traits
// Provides: {"InvertMod"}
// Dependencies: {}
# [doc = " Compute `1 / self mod p`."] pub trait InvertMod < Mod = NonZero < Self > > : Sized { # [doc = " Output type."] type Output ; # [doc = " Compute `1 / self mod p`."] fn invert_mod (& self , p : & Mod) -> CtOption < Self :: Output > ; }
};
}
