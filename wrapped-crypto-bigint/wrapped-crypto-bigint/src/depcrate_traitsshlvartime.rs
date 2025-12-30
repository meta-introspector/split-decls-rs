// Generated macro for ShlVartime (trait)
macro_rules! Depcrate_traitsShlVartime {
() => {
// Module: crate::traits
// Provides: {"ShlVartime"}
// Dependencies: {}
# [doc = " Left shifts, variable time in `shift`."] pub trait ShlVartime : Sized { # [doc = " Computes `self << shift`."] # [doc = ""] # [doc = " Returns `None` if `shift >= self.bits_precision()`."] fn overflowing_shl_vartime (& self , shift : u32) -> CtOption < Self > ; # [doc = " Computes `self << shift` in a panic-free manner, masking off bits of `shift`"] # [doc = " which would cause the shift to exceed the type's width."] fn wrapping_shl_vartime (& self , shift : u32) -> Self ; }
};
}
