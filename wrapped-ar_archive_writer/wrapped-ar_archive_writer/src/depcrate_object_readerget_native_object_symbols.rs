// Generated macro for get_native_object_symbols (function)
macro_rules! Depcrate_object_readerget_native_object_symbols {
() => {
// Module: crate::object_reader
// Provides: {"get_native_object_symbols"}
// Dependencies: {}
pub fn get_native_object_symbols (buf : & [u8] , f : & mut dyn FnMut (& [u8]) -> io :: Result < () > ,) -> io :: Result < bool > { match object :: File :: parse (buf) { Ok (file) => { for sym in file . symbols () { if ! is_archive_symbol (& sym) { continue ; } f (sym . name_bytes () . expect ("FIXME")) ? ; } Ok (true) } Err (_) => { let mut offset = 0 ; if ImportObjectHeader :: parse (buf , & mut offset) . is_ok () { coff_import_file :: get_short_import_symbol (buf , f) . or (Ok (false)) } else { Ok (false) } } } }
};
}
