// Generated macro for impl_135 (impl)
macro_rules! Depcrate_unitimpl_135 {
() => {
// Module: crate::unit
// Provides: {"impl_135"}
// Dependencies: {}
impl < R : gimli :: Reader > SupUnits < R > { pub (crate) fn parse (sections : & gimli :: Dwarf < R >) -> Result < Self , Error > { let mut sup_units = Vec :: new () ; let mut units = sections . units () ; while let Some (header) = units . next () ? { let offset = match header . offset () . as_debug_info_offset () { Some (offset) => offset , None => continue , } ; let dw_unit = match sections . unit (header) { Ok (dw_unit) => dw_unit , Err (_) => continue , } ; sup_units . push (SupUnit { dw_unit , offset }) ; } Ok (SupUnits { units : sup_units . into_boxed_slice () , }) } pub (crate) fn find_offset (& self , offset : gimli :: DebugInfoOffset < R :: Offset > ,) -> Result < & gimli :: Unit < R > , Error > { match self . units . binary_search_by_key (& offset . 0 , | unit | unit . offset . 0) { Ok (_) | Err (0) => Err (gimli :: Error :: NoEntryAtGivenOffset) , Err (i) => Ok (& self . units [i - 1] . dw_unit) , } } }
};
}
