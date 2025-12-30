// Generated macro for add_upstream_native_libraries (function)
macro_rules! Depcrate_back_linkadd_upstream_native_libraries {
() => {
// Module: crate::back::link
// Provides: {"add_upstream_native_libraries"}
// Dependencies: {}
fn add_upstream_native_libraries (cmd : & mut dyn Linker , sess : & Session , archive_builder_builder : & dyn ArchiveBuilderBuilder , codegen_results : & CodegenResults , tmpdir : & Path , link_output_kind : LinkOutputKind ,) { for & cnum in & codegen_results . crate_info . used_crates { let link_static = false ; let link_dynamic = true ; add_native_libs_from_crate (cmd , sess , archive_builder_builder , codegen_results , tmpdir , & Default :: default () , cnum , link_static , link_dynamic , link_output_kind ,) ; } }
};
}
