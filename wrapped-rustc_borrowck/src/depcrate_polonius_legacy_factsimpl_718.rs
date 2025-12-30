// Generated macro for impl_718 (impl)
macro_rules! Depcrate_polonius_legacy_factsimpl_718 {
() => {
// Module: crate::polonius::legacy::facts
// Provides: {"impl_718"}
// Dependencies: {}
impl < 'w > FactWriter < 'w > { fn write_facts_to_path < T > (& self , rows : & [T] , file_name : & str) -> Result < () , Box < dyn Error > > where T : FactRow , { let file = & self . dir . join (file_name) ; let mut file = File :: create_buffered (file) ? ; for row in rows { row . write (& mut file , self . location_table) ? ; } Ok (()) } }
};
}
