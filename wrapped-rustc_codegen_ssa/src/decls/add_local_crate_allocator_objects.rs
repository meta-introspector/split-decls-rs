macro_rules! deps {
    () => {
        CodegenResults!();
        Linker!();
    };
}

macro_rules! add_local_crate_allocator_objects {
    () => {
        deps!();
        # [doc = " Add object files for allocator code linked once for the whole crate tree."] fn add_local_crate_allocator_objects (cmd : & mut dyn Linker , codegen_results : & CodegenResults , crate_type : CrateType ,) { if needs_allocator_shim_for_linking (& codegen_results . crate_info . dependency_formats , crate_type) { if let Some (obj) = codegen_results . allocator_module . as_ref () . and_then (| m | m . object . as_ref ()) { cmd . add_object (obj) ; } } }
    };
}

add_local_crate_allocator_objects!()