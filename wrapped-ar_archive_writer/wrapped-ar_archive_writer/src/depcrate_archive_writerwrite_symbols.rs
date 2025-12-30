// Generated macro for write_symbols (function)
macro_rules! Depcrate_archive_writerwrite_symbols {
() => {
// Module: crate::archive_writer
// Provides: {"write_symbols"}
// Dependencies: {}
fn write_symbols (obj : & [u8] , index : u16 , sym_names : & mut Cursor < Vec < u8 > > , sym_map : & mut Option < & mut SymMap > , object_reader : & ObjectReader ,) -> io :: Result < Vec < u64 > > { let mut ret = vec ! [] ; let mut is_using_map = false ; let (mut map , mut ec_map) = if let Some (sym_map) = sym_map { if sym_map . use_ec_map && (object_reader . is_ec_object_file) (obj) { (Some (& mut sym_map . ec_map) , None) } else { is_using_map = true ; (Some (& mut sym_map . map) , sym_map . use_ec_map . then_some (& mut sym_map . ec_map) ,) } } else { (None , None) } ; (object_reader . get_symbols) (obj , & mut | name | { if let Some (map) = & mut map { let entry = map . entry (name . to_vec () . into_boxed_slice ()) ; if matches ! (entry , std :: collections :: btree_map :: Entry :: Occupied (_)) { return Ok (()) ; } entry . or_insert (index) ; if is_using_map { ret . push (sym_names . stream_position () ?) ; sym_names . write_all (name) ? ; sym_names . write_all (& [0]) ? ; if let Some (ec_map) = & mut ec_map && is_import_descriptor (name) { ec_map . insert (name . to_vec () . into_boxed_slice () , index) ; } } } else { ret . push (sym_names . stream_position () ?) ; sym_names . write_all (name) ? ; sym_names . write_all (& [0]) ? ; } Ok (()) }) ? ; Ok (ret) }
};
}
