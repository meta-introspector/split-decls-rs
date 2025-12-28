macro_rules! deps {
    () => {
        CodegenResults!();
        Linker!();
        ArchiveBuilderBuilder!();
    };
}

macro_rules! add_local_crate_metadata_objects {
    () => {
        deps!();
        # [doc = " Add object files containing metadata for the current crate."] fn add_local_crate_metadata_objects (cmd : & mut dyn Linker , sess : & Session , archive_builder_builder : & dyn ArchiveBuilderBuilder , crate_type : CrateType , tmpdir : & Path , codegen_results : & CodegenResults , metadata : & EncodedMetadata ,) { if matches ! (crate_type , CrateType :: Dylib | CrateType :: ProcMacro) { let data = archive_builder_builder . create_dylib_metadata_wrapper (sess , & metadata , & codegen_results . crate_info . metadata_symbol ,) ; let obj = emit_wrapper_file (sess , & data , tmpdir , "rmeta.o") ; cmd . add_object (& obj) ; } }
    };
}

add_local_crate_metadata_objects!();