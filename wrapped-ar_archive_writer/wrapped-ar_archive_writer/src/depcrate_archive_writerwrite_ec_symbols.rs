// Generated macro for write_ec_symbols (function)
macro_rules! Depcrate_archive_writerwrite_ec_symbols {
() => {
// Module: crate::archive_writer
// Provides: {"write_ec_symbols"}
// Dependencies: {}
fn write_ec_symbols < W : Write + Seek > (w : & mut W , sym_map : & SymMap) -> io :: Result < () > { let (size , pad) = compute_ec_symbols_size_and_pad (sym_map) ; print_gnu_small_member_header (w , "/<ECSYMBOLS>" . to_string () , now () , 0 , 0 , 0 , size) ? ; w . write_all (& u32 :: try_from (sym_map . ec_map . len ()) . unwrap () . to_le_bytes ()) ? ; for s in sym_map . ec_map . values () { w . write_all (& s . to_le_bytes ()) ? ; } for s in sym_map . ec_map . keys () { w . write_all (s) ? ; w . write_all (& [0]) ? ; } write ! (w , "{nil:\0<pad$}" , nil = "" , pad = usize :: try_from (pad) . unwrap ()) ? ; Ok (()) }
};
}
