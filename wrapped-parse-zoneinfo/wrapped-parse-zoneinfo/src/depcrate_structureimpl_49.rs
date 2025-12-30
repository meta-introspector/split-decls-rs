// Generated macro for impl_49 (impl)
macro_rules! Depcrate_structureimpl_49 {
() => {
// Module: crate::structure
// Provides: {"impl_49"}
// Dependencies: {}
impl < 'table > IntoIterator for TableStructure < 'table > { type Item = TableStructureEntry < 'table > ; type IntoIter = Iter < 'table > ; fn into_iter (self) -> Self :: IntoIter { let mut keys : Vec < _ > = self . mappings . keys () . cloned () . collect () ; keys . sort_by (| a , b | b . cmp (a)) ; Iter { structure : self , keys , } } }
};
}
