// Generated macro for Unit (struct)
macro_rules! Depcrate_read_dwarfUnit {
() => {
// Module: crate::read::dwarf
// Provides: {"Unit"}
// Dependencies: {}
# [doc = " All of the commonly used information for a unit in the `.debug_info` or `.debug_types`"] # [doc = " sections."] # [derive (Debug)] pub struct Unit < R , Offset = < R as Reader > :: Offset > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { # [doc = " The header of the unit."] pub header : UnitHeader < R , Offset > , # [doc = " The parsed abbreviations for the unit."] pub abbreviations : Arc < Abbreviations > , # [doc = " The `DW_AT_name` attribute of the unit."] pub name : Option < R > , # [doc = " The `DW_AT_comp_dir` attribute of the unit."] pub comp_dir : Option < R > , # [doc = " The `DW_AT_low_pc` attribute of the unit. Defaults to 0."] pub low_pc : u64 , # [doc = " The `DW_AT_str_offsets_base` attribute of the unit. Defaults to 0."] pub str_offsets_base : DebugStrOffsetsBase < Offset > , # [doc = " The `DW_AT_addr_base` attribute of the unit. Defaults to 0."] pub addr_base : DebugAddrBase < Offset > , # [doc = " The `DW_AT_loclists_base` attribute of the unit. Defaults to 0."] pub loclists_base : DebugLocListsBase < Offset > , # [doc = " The `DW_AT_rnglists_base` attribute of the unit. Defaults to 0."] pub rnglists_base : DebugRngListsBase < Offset > , # [doc = " The line number program of the unit."] pub line_program : Option < IncompleteLineProgram < R , Offset > > , # [doc = " The DWO ID of a skeleton unit or split compilation unit."] pub dwo_id : Option < DwoId > , }
};
}
