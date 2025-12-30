// Generated macro for class (function)
macro_rules! Depcrate_propertiesclass {
() => {
// Module: crate::properties
// Provides: {"class"}
// Dependencies: {}
fn class (ranges : & [(char , char)]) -> CharClass { let ranges = ranges . iter () . cloned () . map (| (c1 , c2) | ClassRange :: new (c1 , c2)) . collect () ; CharClass :: new (ranges) }
};
}
