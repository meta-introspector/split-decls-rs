// Generated macro for dep_symbol_lookup_fn (function)
macro_rules! Depcrate_driver_jitdep_symbol_lookup_fn {
() => {
// Module: crate::driver::jit
// Provides: {"dep_symbol_lookup_fn"}
// Dependencies: {}
fn dep_symbol_lookup_fn (sess : & Session , crate_info : CrateInfo ,) -> Box < dyn Fn (& str) -> Option < * const u8 > + Send > { use rustc_middle :: middle :: dependency_format :: Linkage ; let mut dylib_paths = Vec :: new () ; let data = & crate_info . dependency_formats [& rustc_session :: config :: CrateType :: Executable] ; for & cnum in crate_info . used_crates . iter () . rev () { let src = & crate_info . used_crate_source [& cnum] ; match data [cnum] { Linkage :: NotLinked | Linkage :: IncludedFromDylib => { } Linkage :: Static => { let name = crate_info . crate_name [& cnum] ; let mut diag = sess . dcx () . struct_err (format ! ("Can't load static lib {}" , name)) ; diag . note ("rustc_codegen_cranelift can only load dylibs in JIT mode.") ; diag . emit () ; } Linkage :: Dynamic => { dylib_paths . push (src . dylib . as_ref () . unwrap () . 0 . clone ()) ; } } } let imported_dylibs = Box :: leak (dylib_paths . into_iter () . map (| path | unsafe { libloading :: Library :: new (& path) . unwrap () }) . collect :: < Box < [_] > > () ,) ; sess . dcx () . abort_if_errors () ; Box :: new (move | sym_name | { for dylib in & * imported_dylibs { if let Ok (sym) = unsafe { dylib . get :: < * const u8 > (sym_name . as_bytes ()) } { return Some (* sym) ; } } None }) }
};
}
