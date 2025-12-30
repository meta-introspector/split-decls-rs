// Generated macro for make_module (function)
macro_rules! Depcrate_driver_aotmake_module {
() => {
// Module: crate::driver::aot
// Provides: {"make_module"}
// Dependencies: {}
fn make_module (sess : & Session , name : String) -> UnwindModule < ObjectModule > { let isa = crate :: build_isa (sess , false) ; let mut builder = ObjectBuilder :: new (isa , name + ".o" , cranelift_module :: default_libcall_names ()) . unwrap () ; let default_function_sections = sess . target . function_sections && ! sess . target . is_like_windows ; builder . per_function_section (sess . opts . unstable_opts . function_sections . unwrap_or (default_function_sections) ,) ; UnwindModule :: new (ObjectModule :: new (builder) , true) }
};
}
