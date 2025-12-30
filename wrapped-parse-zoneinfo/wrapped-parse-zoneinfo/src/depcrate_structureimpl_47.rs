// Generated macro for impl_47 (impl)
macro_rules! Depcrate_structureimpl_47 {
() => {
// Module: crate::structure
// Provides: {"impl_47"}
// Dependencies: {}
impl Structure for Table { fn structure (& self) -> TableStructure < '_ > { let mut mappings = BTreeMap :: new () ; for key in self . zonesets . keys () . chain (self . links . keys ()) { let last_slash = match key . rfind ('/') { Some (pos) => pos , None => continue , } ; let parent = & key [.. last_slash] ; { let set = mappings . entry (parent) . or_insert_with (BTreeSet :: new) ; set . insert (Child :: TimeZone (& key [last_slash + 1 ..])) ; } if let Some (first_slash) = parent . find ('/') { let grandparent = & parent [.. first_slash] ; let set = mappings . entry (grandparent) . or_insert_with (BTreeSet :: new) ; set . insert (Child :: Submodule (& parent [first_slash + 1 ..])) ; } } TableStructure { mappings } } }
};
}
