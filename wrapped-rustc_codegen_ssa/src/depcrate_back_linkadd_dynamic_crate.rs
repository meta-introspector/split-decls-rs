// Generated macro for add_dynamic_crate (function)
macro_rules! Depcrate_back_linkadd_dynamic_crate {
() => {
// Module: crate::back::link
// Provides: {"add_dynamic_crate"}
// Dependencies: {}
fn add_dynamic_crate (cmd : & mut dyn Linker , sess : & Session , cratepath : & Path) { cmd . link_dylib_by_path (& rehome_lib_path (sess , cratepath) , true) ; }
};
}
