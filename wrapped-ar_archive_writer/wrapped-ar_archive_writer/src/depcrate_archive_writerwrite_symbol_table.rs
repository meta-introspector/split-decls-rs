// Generated macro for write_symbol_table (function)
macro_rules! Depcrate_archive_writerwrite_symbol_table {
() => {
// Module: crate::archive_writer
// Provides: {"write_symbol_table"}
// Dependencies: {}
fn write_symbol_table < W : Write + Seek > (w : & mut W , kind : ArchiveKind , members : & [MemberData < '_ >] , string_table : & [u8] , members_offset : u64 , num_syms : u64 , prev_member_offset : u64 , next_member_offset : u64 , is_64_bit : bool ,) -> io :: Result < () > { if string_table . is_empty () && ! is_darwin (kind) && ! is_coff_archive (kind) { return Ok (()) ; } let offset_size = if is_64bit_kind (kind) { 8 } else { 4 } ; let (size , pad) = compute_symbol_table_size_and_pad (kind , num_syms , offset_size , string_table . len () . try_into () . unwrap () ,) ; write_symbol_table_header (w , kind , size , prev_member_offset , next_member_offset) ? ; if is_bsd_like (kind) { print_n_bits (w , kind , num_syms * 2 * offset_size) ? ; } else { print_n_bits (w , kind , num_syms) ? ; } let mut pos = members_offset ; for m in members { if is_aix_big_archive (kind) { pos += m . pre_head_pad_size ; if (m . object_reader . is_64_bit_object_file) (m . data) != is_64_bit { pos += u64 :: try_from (m . header . len () + m . data . len () + m . padding . len ()) . unwrap () ; continue ; } } for & string_offset in & m . symbols { if is_bsd_like (kind) { print_n_bits (w , kind , string_offset) ? ; } print_n_bits (w , kind , pos) ? ; } pos += u64 :: try_from (m . header . len () + m . data . len () + m . padding . len ()) . unwrap () ; } if is_bsd_like (kind) { print_n_bits (w , kind , u64 :: try_from (string_table . len ()) . unwrap ()) ? ; } w . write_all (string_table) ? ; write ! (w , "{nil:\0<pad$}" , nil = "" , pad = usize :: try_from (pad) . unwrap ()) }
};
}
