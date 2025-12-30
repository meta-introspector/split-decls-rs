// Generated macro for write_symbol_table_header (function)
macro_rules! Depcrate_archive_writerwrite_symbol_table_header {
() => {
// Module: crate::archive_writer
// Provides: {"write_symbol_table_header"}
// Dependencies: {}
fn write_symbol_table_header < W : Write + Seek > (w : & mut W , kind : ArchiveKind , size : u64 , prev_member_offset : u64 , next_member_offset : u64 ,) -> io :: Result < () > { if is_bsd_like (kind) { let name = if is_64bit_kind (kind) { "__.SYMDEF_64" } else { "__.SYMDEF" } ; let pos = w . stream_position () ? ; print_bsd_member_header (w , pos , name , now () , 0 , 0 , 0 , size) } else if is_aix_big_archive (kind) { print_big_archive_member_header (w , "" , now () , 0 , 0 , 0 , size , prev_member_offset , next_member_offset ,) } else { let name = if is_64bit_kind (kind) { "/SYM64" } else { "" } ; print_gnu_small_member_header (w , name . to_string () , now () , 0 , 0 , 0 , size) } }
};
}
