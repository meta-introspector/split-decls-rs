// Generated macro for impl_1155 (impl)
macro_rules! Depcrate_test_runner_failure_persistence_mapimpl_1155 {
() => {
// Module: crate::test_runner::failure_persistence::map
// Provides: {"impl_1155"}
// Dependencies: {}
impl FailurePersistence for MapFailurePersistence { fn load_persisted_failures2 (& self , source_file : Option < & 'static str > ,) -> Vec < PersistedSeed > { source_file . and_then (| source | self . map . get (source)) . map (| seeds | seeds . iter () . cloned () . collect :: < Vec < _ > > ()) . unwrap_or_default () } fn save_persisted_failure2 (& mut self , source_file : Option < & 'static str > , seed : PersistedSeed , _shrunken_value : & dyn fmt :: Debug ,) { let s = match source_file { Some (sf) => sf , None => return , } ; let set = self . map . entry (s) . or_insert_with (BTreeSet :: new) ; set . insert (seed) ; } fn box_clone (& self) -> Box < dyn FailurePersistence > { Box :: new (self . clone ()) } fn eq (& self , other : & dyn FailurePersistence) -> bool { other . as_any () . downcast_ref :: < Self > () . map_or (false , | x | x == self) } fn as_any (& self) -> & dyn Any { self } }
};
}
