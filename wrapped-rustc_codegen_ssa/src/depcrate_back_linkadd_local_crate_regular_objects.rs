// Generated macro for add_local_crate_regular_objects (function)
macro_rules! Depcrate_back_linkadd_local_crate_regular_objects {
() => {
// Module: crate::back::link
// Provides: {"add_local_crate_regular_objects"}
// Dependencies: {}
# [doc = " Add object files containing code from the current crate."] fn add_local_crate_regular_objects (cmd : & mut dyn Linker , codegen_results : & CodegenResults) { for obj in codegen_results . modules . iter () . filter_map (| m | m . object . as_ref ()) { cmd . add_object (obj) ; } }
};
}
