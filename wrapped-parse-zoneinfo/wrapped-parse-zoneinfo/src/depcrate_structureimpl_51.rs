// Generated macro for impl_51 (impl)
macro_rules! Depcrate_structureimpl_51 {
() => {
// Module: crate::structure
// Provides: {"impl_51"}
// Dependencies: {}
impl < 'table > Iterator for Iter < 'table > { type Item = TableStructureEntry < 'table > ; fn next (& mut self) -> Option < Self :: Item > { let key = self . keys . pop () ? ; let values = self . structure . mappings [key] . iter () . cloned () . collect () ; Some (TableStructureEntry { name : key , children : values , }) } }
};
}
