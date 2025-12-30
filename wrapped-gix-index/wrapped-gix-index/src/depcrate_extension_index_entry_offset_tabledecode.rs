// Generated macro for decode (function)
macro_rules! Depcrate_extension_index_entry_offset_tabledecode {
() => {
// Module: crate::extension::index_entry_offset_table
// Provides: {"decode"}
// Dependencies: {}
pub fn decode (data : & [u8]) -> Option < Vec < Offset > > { let (version , mut data) = read_u32 (data) ? ; match version { 1 => { } _unknown => return None , } let entry_size = 4 + 4 ; let num_offsets = data . len () / entry_size ; if num_offsets == 0 || data . len () % entry_size != 0 { return None ; } let mut out = Vec :: with_capacity (entry_size) ; for _ in 0 .. num_offsets { let (offset , chunk) = read_u32 (data) ? ; let (num_entries , chunk) = read_u32 (chunk) ? ; out . push (Offset { from_beginning_of_file : offset , num_entries , }) ; data = chunk ; } debug_assert ! (data . is_empty ()) ; out . into () }
};
}
