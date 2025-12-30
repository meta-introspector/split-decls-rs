// Generated macro for get_short_import_symbol (function)
macro_rules! Depcrate_coff_import_fileget_short_import_symbol {
() => {
// Module: crate::coff_import_file
// Provides: {"get_short_import_symbol"}
// Dependencies: {}
pub (crate) fn get_short_import_symbol (buf : & [u8] , f : & mut dyn FnMut (& [u8]) -> Result < () > ,) -> Result < bool > { let mut offset = 0 ; let header = ImportObjectHeader :: parse (buf , & mut offset) . map_err (Error :: other) ? ; let data = header . parse_data (buf , & mut offset) . map_err (Error :: other) ? ; let is_ec = header . machine . get (object :: LittleEndian) == object :: pe :: IMAGE_FILE_MACHINE_ARM64EC ; let name = data . symbol () ; let demangled_name = is_ec . then (| | get_arm64ec_demangled_function_name (from_utf8 (name) . unwrap ())) . flatten () . map_or_else (| | Cow :: Borrowed (name) , | demangled_name | Cow :: Owned (demangled_name . into_bytes ()) ,) ; const IMP_PREFIX : & [u8] = b"__imp_" ; f (& IMP_PREFIX . iter () . chain (demangled_name . as_ref ()) . copied () . collect :: < Vec < _ > > ()) ? ; if header . import_type () == ImportType :: Data . into () { return Ok (true) ; } f (demangled_name . as_ref ()) ? ; if header . machine . get (object :: LittleEndian) == object :: pe :: IMAGE_FILE_MACHINE_ARM64EC { const IMP_PREFIX : & [u8] = b"__imp_aux_" ; f (& IMP_PREFIX . iter () . chain (demangled_name . as_ref ()) . copied () . collect :: < Vec < _ > > ()) ? ; f (name) ? ; } Ok (true) }
};
}
