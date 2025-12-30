// Generated macro for macro_1123 (macro)
macro_rules! Depcrate_core_config_toml_distmacro_1123 {
() => {
// Module: crate::core::config::toml::dist
// Provides: {"macro_1123"}
// Dependencies: {}
define_config ! { # [derive (Default)] struct Dist { sign_folder : Option < String > = "sign-folder" , upload_addr : Option < String > = "upload-addr" , src_tarball : Option < bool > = "src-tarball" , compression_formats : Option < Vec < String >> = "compression-formats" , compression_profile : Option < String > = "compression-profile" , include_mingw_linker : Option < bool > = "include-mingw-linker" , vendor : Option < bool > = "vendor" , } }
};
}
