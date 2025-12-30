// Generated macro for ignore_backends (function)
macro_rules! Depcrate_directivesignore_backends {
() => {
// Module: crate::directives
// Provides: {"ignore_backends"}
// Dependencies: {}
fn ignore_backends (config : & Config , path : & Utf8Path , line : & str , line_number : usize ,) -> IgnoreDecision { if let Some (backends_to_ignore) = config . parse_name_value_directive (line , "ignore-backends" , path , line_number) { for backend in backends_to_ignore . split_whitespace () . map (| backend | { match CodegenBackend :: try_from (backend) { Ok (backend) => backend , Err (error) => { panic ! ("Invalid ignore-backends value `{backend}` in `{path}`: {error}") } } }) { if config . default_codegen_backend == backend { return IgnoreDecision :: Ignore { reason : format ! ("{} backend is marked as ignore" , backend . as_str ()) , } ; } } } IgnoreDecision :: Continue }
};
}
