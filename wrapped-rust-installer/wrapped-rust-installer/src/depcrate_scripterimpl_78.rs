// Generated macro for impl_78 (impl)
macro_rules! Depcrate_scripterimpl_78 {
() => {
// Module: crate::scripter
// Provides: {"impl_78"}
// Dependencies: {}
impl Scripter { # [doc = " Generates the actual installer script"] pub fn run (self) -> Result < () > { let product_name = self . product_name . replace ('-' , " ") ; let success_message = self . success_message . replace ('-' , " ") ; let script = TEMPLATE . replace ("%%TEMPLATE_PRODUCT_NAME%%" , & sh_quote (& product_name)) . replace ("%%TEMPLATE_REL_MANIFEST_DIR%%" , & self . rel_manifest_dir) . replace ("%%TEMPLATE_SUCCESS_MESSAGE%%" , & sh_quote (& success_message)) . replace ("%%TEMPLATE_LEGACY_MANIFEST_DIRS%%" , & sh_quote (& self . legacy_manifest_dirs)) . replace ("%%TEMPLATE_RUST_INSTALLER_VERSION%%" , & sh_quote (& crate :: RUST_INSTALLER_VERSION) ,) ; create_new_executable (& self . output_script) ? . write_all (script . as_ref ()) . with_context (| | format ! ("failed to write output script '{}'" , self . output_script)) ? ; Ok (()) } }
};
}
