// Generated macro for DerOrd (trait)
macro_rules! Depcrate_ordDerOrd {
() => {
// Module: crate::ord
// Provides: {"DerOrd"}
// Dependencies: {}
# [doc = " DER ordering trait."] # [doc = ""] # [doc = " Compares the ordering of two values based on their ASN.1 DER"] # [doc = " serializations."] # [doc = ""] # [doc = " This is used by the DER encoding for `SET OF` in order to establish an"] # [doc = " ordering for the elements of sets."] pub trait DerOrd { # [doc = " Return an [`Ordering`] between `self` and `other` when serialized as"] # [doc = " ASN.1 DER."] fn der_cmp (& self , other : & Self) -> Result < Ordering > ; }
};
}
