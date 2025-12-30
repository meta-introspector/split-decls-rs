// Generated macro for prepare_lto (function)
macro_rules! Depcrate_back_ltoprepare_lto {
() => {
// Module: crate::back::lto
// Provides: {"prepare_lto"}
// Dependencies: {}
fn prepare_lto (cgcx : & CodegenContext < GccCodegenBackend > , each_linked_rlib_for_lto : & [PathBuf] , dcx : DiagCtxtHandle < '_ > ,) -> LtoData { let tmp_path = match tempdir () { Ok (tmp_path) => tmp_path , Err (error) => { dcx . fatal (format ! ("Cannot create temporary directory: {}" , error)) ; } } ; let mut upstream_modules = Vec :: new () ; if cgcx . lto != Lto :: ThinLocal { for path in each_linked_rlib_for_lto { let archive_data = unsafe { Mmap :: map (File :: open (path) . expect ("couldn't open rlib")) . expect ("couldn't map rlib") } ; let archive = ArchiveFile :: parse (& * archive_data) . expect ("wanted an rlib") ; let obj_files = archive . members () . filter_map (| child | { child . ok () . and_then (| c | { std :: str :: from_utf8 (c . name ()) . ok () . map (| name | (name . trim () , c)) }) }) . filter (| & (name , _) | looks_like_rust_object_file (name)) ; for (name , child) in obj_files { info ! ("adding bitcode from {}" , name) ; let path = tmp_path . path () . join (name) ; match save_as_file (child . data (& * archive_data) . expect ("corrupt rlib") , & path) { Ok (()) => { let buffer = ModuleBuffer :: new (path) ; let module = SerializedModule :: Local (buffer) ; upstream_modules . push ((module , CString :: new (name) . unwrap ())) ; } Err (e) => { dcx . emit_fatal (e) ; } } } } } LtoData { upstream_modules , tmp_path } }
};
}
