// Generated macro for strip_with_external_utility (function)
macro_rules! Depcrate_back_linkstrip_with_external_utility {
() => {
// Module: crate::back::link
// Provides: {"strip_with_external_utility"}
// Dependencies: {}
fn strip_with_external_utility (sess : & Session , util : & str , out_filename : & Path , options : & [& str]) { let mut cmd = Command :: new (util) ; cmd . args (options) ; let mut new_path = sess . get_tools_search_paths (false) ; if let Some (path) = env :: var_os ("PATH") { new_path . extend (env :: split_paths (& path)) ; } cmd . env ("PATH" , env :: join_paths (new_path) . unwrap ()) ; let prog = cmd . arg (out_filename) . output () ; match prog { Ok (prog) => { if ! prog . status . success () { let mut output = prog . stderr . clone () ; output . extend_from_slice (& prog . stdout) ; sess . dcx () . emit_warn (errors :: StrippingDebugInfoFailed { util , status : prog . status , output : escape_string (& output) , }) ; } } Err (error) => sess . dcx () . emit_fatal (errors :: UnableToRun { util , error }) , } }
};
}
