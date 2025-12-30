// Generated macro for add_local_crate_allocator_objects (function)
macro_rules! Depcrate_back_linkadd_local_crate_allocator_objects {
() => {
// Module: crate::back::link
// Provides: {"add_local_crate_allocator_objects"}
// Dependencies: {}
# [doc = " Add object files for allocator code linked once for the whole crate tree."] fn add_local_crate_allocator_objects (cmd : & mut dyn Linker , codegen_results : & CodegenResults , crate_type : CrateType ,) { if needs_allocator_shim_for_linking (& codegen_results . crate_info . dependency_formats , crate_type) { if let Some (obj) = codegen_results . allocator_module . as_ref () . and_then (| m | m . object . as_ref ()) { cmd . add_object (obj) ; } } }
};
}
