// Generated macro for impl_22 (impl)
macro_rules! Depcrate_dylibimpl_22 {
() => {
// Module: crate::dylib
// Provides: {"impl_22"}
// Dependencies: {}
impl ProcMacroLibrary { fn open (path : & Utf8Path) -> Result < Self , LoadProcMacroDylibError > { let file = fs :: File :: open (path) ? ; # [allow (clippy :: undocumented_unsafe_blocks)] let file = unsafe { memmap2 :: Mmap :: map (& file) } ? ; let obj = object :: File :: parse (& * file) . map_err (| e | io :: Error :: new (io :: ErrorKind :: InvalidData , e)) ? ; let version_info = version :: read_dylib_info (& obj) ? ; if version_info . version_string != crate :: RUSTC_VERSION_STRING { return Err (LoadProcMacroDylibError :: AbiMismatch (version_info . version_string)) ; } let symbol_name = find_registrar_symbol (& obj) . map_err (invalid_data_err) ? . ok_or_else (| | { invalid_data_err (format ! ("Cannot find registrar symbol in file {path}")) }) ? ; let lib = unsafe { load_library (path) } . map_err (invalid_data_err) ? ; let proc_macros = unsafe { lib . get :: < & 'static & 'static ProcMacros > (symbol_name . as_bytes ()) } ; match proc_macros { Ok (proc_macros) => Ok (ProcMacroLibrary { proc_macros : * proc_macros , _lib : lib }) , Err (e) => Err (e . into ()) , } } }
};
}
