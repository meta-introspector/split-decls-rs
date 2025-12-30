// Generated macro for impl_814 (impl)
macro_rules! Depcrate_offset_local_tz_info_parserimpl_814 {
() => {
// Module: crate::offset::local::tz_info::parser
// Provides: {"impl_814"}
// Dependencies: {}
impl Header { fn new (cursor : & mut Cursor) -> Result < Self , Error > { let magic = cursor . read_exact (4) ? ; if magic != * b"TZif" { return Err (Error :: InvalidTzFile ("invalid magic number")) ; } let version = match cursor . read_exact (1) ? { [0x00] => Version :: V1 , [0x32] => Version :: V2 , [0x33] => Version :: V3 , _ => return Err (Error :: UnsupportedTzFile ("unsupported TZif version")) , } ; cursor . read_exact (15) ? ; let ut_local_count = cursor . read_be_u32 () ? ; let std_wall_count = cursor . read_be_u32 () ? ; let leap_count = cursor . read_be_u32 () ? ; let transition_count = cursor . read_be_u32 () ? ; let type_count = cursor . read_be_u32 () ? ; let char_count = cursor . read_be_u32 () ? ; if ! (type_count != 0 && char_count != 0 && (ut_local_count == 0 || ut_local_count == type_count) && (std_wall_count == 0 || std_wall_count == type_count)) { return Err (Error :: InvalidTzFile ("invalid header")) ; } Ok (Self { version , ut_local_count : ut_local_count as usize , std_wall_count : std_wall_count as usize , leap_count : leap_count as usize , transition_count : transition_count as usize , type_count : type_count as usize , char_count : char_count as usize , }) } }
};
}
