// Generated macro for Unit (struct)
macro_rules! Depcrate_write_unitUnit {
() => {
// Module: crate::write::unit
// Provides: {"Unit"}
// Dependencies: {}
# [doc = " A unit's debugging information."] # [derive (Debug)] pub struct Unit { base_id : BaseId , # [doc = " The encoding parameters for this unit."] encoding : Encoding , # [doc = " The line number program for this unit."] pub line_program : LineProgram , # [doc = " A table of range lists used by this unit."] pub ranges : RangeListTable , # [doc = " A table of location lists used by this unit."] pub locations : LocationListTable , # [doc = " All entries in this unit. The order is unrelated to the tree order."] entries : Vec < DebuggingInformationEntry > , # [doc = " The total number of entries, including reserved entries."] # [doc = ""] # [doc = " This may be greater than `entries.len()`."] reserved : usize , # [doc = " The index of the root entry in entries."] root : UnitEntryId , # [doc = " The unit has been written to the output sections."] written : bool , # [doc = " The section offsets for the unit and DIEs after being written."] offsets : UnitOffsets , }
};
}
