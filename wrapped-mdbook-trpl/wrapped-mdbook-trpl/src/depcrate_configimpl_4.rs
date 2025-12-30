// Generated macro for impl_4 (impl)
macro_rules! Depcrate_configimpl_4 {
() => {
// Module: crate::config
// Provides: {"impl_4"}
// Dependencies: {}
impl Mode { pub fn from_context (ctx : & PreprocessorContext , preprocessor_name : & str ,) -> Result < Mode , Error > { let config = ctx . config . get_preprocessor (preprocessor_name) . ok_or_else (| | Error :: NoConfig (preprocessor_name . into ())) ? ; let key = String :: from ("output-mode") ; let mode = config . get (& key) . map (| value | match value . as_str () { Some (s) => Mode :: try_from (s) . map_err (| _ | Error :: BadValue { key , value : value . to_string () , }) , None => Err (Error :: BadValue { key , value : value . to_string () , }) , }) . transpose () ? . unwrap_or (Mode :: Default) ; Ok (mode) } }
};
}
