// Generated macro for read (function)
macro_rules! Depcrate_cargo_config_fileread {
() => {
// Module: crate::cargo_config_file
// Provides: {"read"}
// Dependencies: {}
pub (crate) fn read (manifest : & ManifestPath , extra_env : & FxHashMap < String , Option < String > > , sysroot : & Sysroot ,) -> Option < CargoConfigFile > { let mut cargo_config = sysroot . tool (Tool :: Cargo , manifest . parent () , extra_env) ; cargo_config . args (["-Z" , "unstable-options" , "config" , "get" , "--format" , "json"]) . env ("RUSTC_BOOTSTRAP" , "1") ; if manifest . is_rust_manifest () { cargo_config . arg ("-Zscript") ; } tracing :: debug ! ("Discovering cargo config by {:?}" , cargo_config) ; let json : serde_json :: Map < String , serde_json :: Value > = utf8_stdout (& mut cargo_config) . inspect (| json | { tracing :: debug ! ("Discovered cargo config: {:?}" , json) ; }) . inspect_err (| err | { tracing :: debug ! ("Failed to discover cargo config: {:?}" , err) ; }) . ok () . and_then (| stdout | serde_json :: from_str (& stdout) . ok ()) ? ; Some (json) }
};
}
