// Generated macro for Opts (struct)
macro_rules! DepcrateOpts {
() => {
// Module: crate
// Provides: {"Opts"}
// Dependencies: {}
# [derive (Default , Debug , Clone)] # [cfg_attr (feature = "clap" , derive (clap :: Args))] pub struct Opts { # [cfg_attr (feature = "clap" , arg (long , default_value_t = StringEncoding :: default ()))] pub string_encoding : StringEncoding , # [doc = " Whether or not to generate a stub class for exported functions"] # [cfg_attr (feature = "clap" , arg (long))] pub generate_stub : bool , # [cfg_attr (feature = "clap" , arg (short , long , value_enum))] pub runtime : CSharpRuntime , # [doc = " Use the `internal` access modifier by default instead of `public`"] # [cfg_attr (feature = "clap" , arg (long))] pub internal : bool , # [doc = " Skip generating `cabi_realloc`, `WasmImportLinkageAttribute`, and component type files"] # [cfg_attr (feature = "clap" , arg (long))] pub skip_support_files : bool , # [doc = " Generate code for WIT `Result` types instead of exceptions"] # [cfg_attr (feature = "clap" , arg (long))] pub with_wit_results : bool , }
};
}
