// Generated macro for ScopedMap (struct)
macro_rules! Depcrate_adt_mapScopedMap {
() => {
// Module: crate::adt::map
// Provides: {"ScopedMap"}
// Dependencies: {}
# [doc = " Scoped map that supports inserting and removing lots of key-val pairs"] # [doc = " at once."] # [derive (Debug)] pub struct ScopedMap < K , V > { stack : Vec < Vec < (K , V) > > , }
};
}
