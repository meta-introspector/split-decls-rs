// Generated macro for DataStore (trait)
macro_rules! Depcrate_bakedDataStore {
() => {
// Module: crate::baked
// Provides: {"DataStore"}
// Dependencies: {}
# [doc = " A backing store for baked data"] pub trait DataStore < M : DataMarker > : private :: Sealed { # [doc = " Get the value for a key"] fn get (& self , req : DataIdentifierBorrowed , attributes_prefix_match : bool ,) -> Option < DataPayload < M > > ; # [doc = " The type returned by the iterator"] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [cfg (feature = "alloc")] type IterReturn : Iterator < Item = crate :: prelude :: DataIdentifierCow < 'static > > ; # [doc = " Iterate over all data"] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [cfg (feature = "alloc")] fn iter (& 'static self) -> Self :: IterReturn ; }
};
}
