// Generated macro for combine_libraries (function)
macro_rules! Depcrate_librariescombine_libraries {
() => {
// Module: crate::libraries
// Provides: {"combine_libraries"}
// Dependencies: {}
fn combine_libraries (reader : & Reader , libraries : & mut BTreeMap < String , BTreeMap < String , CallingConvention > > ,) { for types in reader . values () { for ty in types . values () { let Some (ty) = cpp_fn (ty) else { continue ; } ; let library = ty . method . module_name () ; let impl_map = ty . method . impl_map () . unwrap () ; let flags = impl_map . flags () ; let name = impl_map . import_name () . to_string () ; if flags . contains (PInvokeAttributes :: CallConvPlatformapi) { let arches = ty . method . arches () ; let params = if (arches == 0) || (arches & 1 == 1) { ty . method . signature (ty . namespace , & []) . size () } else { 0 } ; libraries . entry (library) . or_default () . insert (name , CallingConvention :: Stdcall (params)) ; } else if flags . contains (PInvokeAttributes :: CallConvCdecl) { libraries . entry (library) . or_default () . insert (name , CallingConvention :: Cdecl) ; } else { panic ! () ; } } } }
};
}
