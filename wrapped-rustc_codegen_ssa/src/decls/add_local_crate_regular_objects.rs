macro_rules! deps {
    () => {
        Linker!();
        CodegenResults!();
    };
}

macro_rules! add_local_crate_regular_objects {
    () => {
        deps!();
        # [doc = " Add object files containing code from the current crate."] fn add_local_crate_regular_objects (cmd : & mut dyn Linker , codegen_results : & CodegenResults) { for obj in codegen_results . modules . iter () . filter_map (| m | m . object . as_ref ()) { cmd . add_object (obj) ; } }
    };
}

add_local_crate_regular_objects!();