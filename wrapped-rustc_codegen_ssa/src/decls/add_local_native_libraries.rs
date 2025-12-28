macro_rules! deps {
    () => {
        CodegenResults!();
        Linker!();
        ArchiveBuilderBuilder!();
    };
}

macro_rules! add_local_native_libraries {
    () => {
        deps!();
        fn add_local_native_libraries (cmd : & mut dyn Linker , sess : & Session , archive_builder_builder : & dyn ArchiveBuilderBuilder , codegen_results : & CodegenResults , tmpdir : & Path , link_output_kind : LinkOutputKind ,) { let link_static = true ; let link_dynamic = true ; add_native_libs_from_crate (cmd , sess , archive_builder_builder , codegen_results , tmpdir , & Default :: default () , LOCAL_CRATE , link_static , link_dynamic , link_output_kind ,) ; }
    };
}

add_local_native_libraries!()