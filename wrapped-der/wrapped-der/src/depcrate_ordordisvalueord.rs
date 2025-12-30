// Generated macro for OrdIsValueOrd (trait)
macro_rules! Depcrate_ordOrdIsValueOrd {
() => {
// Module: crate::ord
// Provides: {"OrdIsValueOrd"}
// Dependencies: {}
# [doc = " Marker trait for types whose `Ord` impl can be used as `ValueOrd`."] # [doc = ""] # [doc = " This means the `Ord` impl will sort values in the same order as their DER"] # [doc = " encodings."] pub trait OrdIsValueOrd : Ord { }
};
}
