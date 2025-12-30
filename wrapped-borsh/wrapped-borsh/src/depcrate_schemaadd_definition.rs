// Generated macro for add_definition (function)
macro_rules! Depcrate_schemaadd_definition {
() => {
// Module: crate::schema
// Provides: {"add_definition"}
// Dependencies: {}
# [doc = " Helper method to add a single type definition to the map."] pub fn add_definition (declaration : Declaration , definition : Definition , definitions : & mut BTreeMap < Declaration , Definition > ,) { match definitions . entry (declaration) { Entry :: Occupied (occ) => { let existing_def = occ . get () ; assert_eq ! (existing_def , & definition , "Redefining type schema for {}. Types with the same names are not supported." , occ . key ()) ; } Entry :: Vacant (vac) => { vac . insert (definition) ; } } }
};
}
