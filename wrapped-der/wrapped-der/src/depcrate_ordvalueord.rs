// Generated macro for ValueOrd (trait)
macro_rules! Depcrate_ordValueOrd {
() => {
// Module: crate::ord
// Provides: {"ValueOrd"}
// Dependencies: {}
# [doc = " DER value ordering trait."] # [doc = ""] # [doc = " Compares the ordering of the value portion of TLV-encoded DER productions."] pub trait ValueOrd { # [doc = " Return an [`Ordering`] between value portion of TLV-encoded `self` and"] # [doc = " `other` when serialized as ASN.1 DER."] fn value_cmp (& self , other : & Self) -> Result < Ordering > ; }
};
}
