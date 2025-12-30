// Generated macro for impl_783 (impl)
macro_rules! Depcrate_read_unitimpl_783 {
() => {
// Module: crate::read::unit
// Provides: {"impl_783"}
// Dependencies: {}
# [doc = " Static methods."] impl < R , Offset > UnitHeader < R , Offset > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { # [doc = " Construct a new `UnitHeader`."] pub fn new (encoding : Encoding , unit_length : Offset , unit_type : UnitType < Offset > , debug_abbrev_offset : DebugAbbrevOffset < Offset > , section : SectionId , unit_offset : UnitSectionOffset < Offset > , entries_buf : R ,) -> Self { UnitHeader { encoding , unit_length , unit_type , debug_abbrev_offset , section , unit_offset , entries_buf , } } }
};
}
