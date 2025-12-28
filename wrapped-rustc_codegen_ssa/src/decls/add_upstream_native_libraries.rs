macro_rules! deps {
    () => {
        ArchiveBuilderBuilder!();
        Linker!();
        CodegenResults!();
    };
}

macro_rules! add_upstream_native_libraries {
    () => {
        deps!();
        fn add_upstream_native_libraries (cmd : & mut dyn Linker , sess : & Session , archive_builder_builder : & dyn ArchiveBuilderBuilder , codegen_results : & CodegenResults , tmpdir : & Path , link_output_kind : LinkOutputKind ,) { for & cnum in & codegen_results . crate_info . used_crates { let link_static = false ; let link_dynamic = true ; add_native_libs_from_crate (cmd , sess , archive_builder_builder , codegen_results , tmpdir , & Default :: default () , cnum , link_static , link_dynamic , link_output_kind ,) ; } }
    };
}

add_upstream_native_libraries!();