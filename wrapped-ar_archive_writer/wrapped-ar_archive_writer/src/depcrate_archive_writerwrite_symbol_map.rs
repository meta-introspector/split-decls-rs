// Generated macro for write_symbol_map (function)
macro_rules! Depcrate_archive_writerwrite_symbol_map {
() => {
// Module: crate::archive_writer
// Provides: {"write_symbol_map"}
// Dependencies: {}
fn write_symbol_map < W : Write + Seek > (w : & mut W , kind : ArchiveKind , members : & [MemberData < '_ >] , sym_map : & SymMap , members_offset : u64 ,) -> io :: Result < () > { let (size , pad) = compute_symbol_map_size_and_pad (members . len () , sym_map) ; write_symbol_table_header (w , kind , size , 0 , 0) ? ; let mut pos : u32 = members_offset . try_into () . unwrap () ; w . write_all (& u32 :: try_from (members . len ()) . unwrap () . to_le_bytes ()) ? ; for m in members { w . write_all (& pos . to_le_bytes ()) ? ; pos = pos . checked_add ((m . header . len () + m . data . len () + m . padding . len ()) . try_into () . unwrap () ,) . unwrap () ; } w . write_all (& u32 :: try_from (sym_map . map . len ()) . unwrap () . to_le_bytes ()) ? ; for s in sym_map . map . values () { w . write_all (& s . to_le_bytes ()) ? ; } for s in sym_map . map . keys () { w . write_all (s) ? ; w . write_all (& [0]) ? ; } write ! (w , "{nil:\0<pad$}" , nil = "" , pad = usize :: try_from (pad) . unwrap ()) ? ; Ok (()) }
};
}
