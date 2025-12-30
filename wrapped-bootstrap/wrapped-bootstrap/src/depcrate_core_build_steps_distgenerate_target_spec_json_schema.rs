// Generated macro for generate_target_spec_json_schema (function)
macro_rules! Depcrate_core_build_steps_distgenerate_target_spec_json_schema {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"generate_target_spec_json_schema"}
// Dependencies: {}
fn generate_target_spec_json_schema (builder : & Builder < '_ > , sysroot : & Path) { let stage1_host = builder . compiler (1 , builder . host_target) ; let mut rustc = builder . rustc_cmd (stage1_host) . fail_fast () ; rustc . env ("RUSTC_BOOTSTRAP" , "1") . args (["--print=target-spec-json-schema" , "-Zunstable-options"]) ; let schema = rustc . run_capture (builder) . stdout () ; let schema_dir = tmpdir (builder) ; t ! (fs :: create_dir_all (& schema_dir)) ; let schema_file = schema_dir . join ("target-spec-json-schema.json") ; t ! (std :: fs :: write (& schema_file , schema)) ; let dst = sysroot . join ("etc") ; t ! (fs :: create_dir_all (& dst)) ; builder . install (& schema_file , & dst , FileType :: Regular) ; }
};
}
