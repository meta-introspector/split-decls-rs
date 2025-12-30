// Generated macro for allow_section_offset (function)
macro_rules! Depcrate_read_unitallow_section_offset {
() => {
// Module: crate::read::unit
// Provides: {"allow_section_offset"}
// Dependencies: {}
fn allow_section_offset (name : constants :: DwAt , version : u16) -> bool { match name { constants :: DW_AT_location | constants :: DW_AT_stmt_list | constants :: DW_AT_string_length | constants :: DW_AT_return_addr | constants :: DW_AT_start_scope | constants :: DW_AT_frame_base | constants :: DW_AT_macro_info | constants :: DW_AT_macros | constants :: DW_AT_segment | constants :: DW_AT_static_link | constants :: DW_AT_use_location | constants :: DW_AT_vtable_elem_location | constants :: DW_AT_ranges => true , constants :: DW_AT_data_member_location => version == 2 || version == 3 , _ => false , } }
};
}
