// Generated macro for impl_61 (impl)
macro_rules! Depcrate_cargo_config_fileimpl_61 {
() => {
// Module: crate::cargo_config_file
// Provides: {"impl_61"}
// Dependencies: {}
impl CargoConfigFile { pub (crate) fn load (manifest : & ManifestPath , extra_env : & FxHashMap < String , Option < String > > , sysroot : & Sysroot ,) -> Option < Self > { let mut cargo_config = sysroot . tool (Tool :: Cargo , manifest . parent () , extra_env) ; cargo_config . args (["-Z" , "unstable-options" , "config" , "get" , "--format" , "toml" , "--show-origin"]) . env ("RUSTC_BOOTSTRAP" , "1") ; if manifest . is_rust_manifest () { cargo_config . arg ("-Zscript") ; } tracing :: debug ! ("Discovering cargo config by {cargo_config:?}") ; utf8_stdout (& mut cargo_config) . inspect (| toml | { tracing :: debug ! ("Discovered cargo config: {toml:?}") ; }) . inspect_err (| err | { tracing :: debug ! ("Failed to discover cargo config: {err:?}") ; }) . ok () . map (CargoConfigFile) } pub (crate) fn read < 'a > (& 'a self) -> Option < CargoConfigFileReader < 'a > > { CargoConfigFileReader :: new (& self . 0) } # [cfg (test)] pub (crate) fn from_string_for_test (s : String) -> Self { CargoConfigFile (s) } }
};
}
