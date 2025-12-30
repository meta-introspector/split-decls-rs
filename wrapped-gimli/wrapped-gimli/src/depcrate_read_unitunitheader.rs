// Generated macro for UnitHeader (struct)
macro_rules! Depcrate_read_unitUnitHeader {
() => {
// Module: crate::read::unit
// Provides: {"UnitHeader"}
// Dependencies: {}
# [doc = " The common fields for the headers of compilation units and"] # [doc = " type units."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct UnitHeader < R , Offset = < R as Reader > :: Offset > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { encoding : Encoding , unit_length : Offset , unit_type : UnitType < Offset > , debug_abbrev_offset : DebugAbbrevOffset < Offset > , section : SectionId , unit_offset : UnitSectionOffset < Offset > , entries_buf : R , }
};
}
