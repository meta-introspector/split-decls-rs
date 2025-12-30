// Generated macro for needs_backends (function)
macro_rules! Depcrate_directivesneeds_backends {
() => {
// Module: crate::directives
// Provides: {"needs_backends"}
// Dependencies: {}
fn needs_backends (config : & Config , path : & Utf8Path , line : & str , line_number : usize ,) -> IgnoreDecision { if let Some (needed_backends) = config . parse_name_value_directive (line , "needs-backends" , path , line_number) { if ! needed_backends . split_whitespace () . map (| backend | match CodegenBackend :: try_from (backend) { Ok (backend) => backend , Err (error) => { panic ! ("Invalid needs-backends value `{backend}` in `{path}`: {error}") } }) . any (| backend | config . default_codegen_backend == backend) { return IgnoreDecision :: Ignore { reason : format ! ("{} backend is not part of required backends" , config . default_codegen_backend . as_str ()) , } ; } } IgnoreDecision :: Continue }
};
}
