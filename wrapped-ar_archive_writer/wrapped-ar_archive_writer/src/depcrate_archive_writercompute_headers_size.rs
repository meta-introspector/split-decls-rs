// Generated macro for compute_headers_size (function)
macro_rules! Depcrate_archive_writercompute_headers_size {
() => {
// Module: crate::archive_writer
// Provides: {"compute_headers_size"}
// Dependencies: {}
fn compute_headers_size (kind : ArchiveKind , num_members : usize , string_member_size : u64 , num_syms : u64 , sym_names_size : u64 , sym_map : Option < & SymMap > ,) -> io :: Result < u64 > { let offset_size = if is_64bit_kind (kind) { 8 } else { 4 } ; let (symtab_size , _) = compute_symbol_table_size_and_pad (kind , num_syms , offset_size , sym_names_size) ; let compute_symbol_table_header_size = | | -> io :: Result < u64 > { let mut tmp = Cursor :: new (Vec :: new ()) ; write_symbol_table_header (& mut tmp , kind , symtab_size , 0 , 0) ? ; Ok (tmp . into_inner () . len () . try_into () . unwrap ()) } ; let header_size = compute_symbol_table_header_size () ? ; let mut size = u64 :: try_from ("!<arch>\n" . len ()) . unwrap () + header_size + symtab_size ; if let Some (sym_map) = sym_map { size += header_size + compute_symbol_map_size_and_pad (num_members , sym_map) . 0 ; if ! sym_map . ec_map . is_empty () { size += header_size + compute_ec_symbols_size_and_pad (sym_map) . 0 ; } } Ok (size + string_member_size) }
};
}
