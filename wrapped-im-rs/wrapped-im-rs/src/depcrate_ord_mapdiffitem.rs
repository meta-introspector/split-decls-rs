// Generated macro for DiffItem (enum)
macro_rules! Depcrate_ord_mapDiffItem {
() => {
// Module: crate::ord::map
// Provides: {"DiffItem"}
// Dependencies: {}
# [doc = " A description of a difference between two ordered maps."] # [derive (PartialEq , Eq , Debug)] pub enum DiffItem < 'a , K , V > { # [doc = " This value has been added to the new map."] Add (& 'a K , & 'a V) , # [doc = " This value has been changed between the two maps."] Update { # [doc = " The old value."] old : (& 'a K , & 'a V) , # [doc = " The new value."] new : (& 'a K , & 'a V) , } , # [doc = " This value has been removed from the new map."] Remove (& 'a K , & 'a V) , }
};
}
