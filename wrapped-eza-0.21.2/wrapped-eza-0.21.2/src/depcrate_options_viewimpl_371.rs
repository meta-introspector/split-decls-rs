// Generated macro for impl_371 (impl)
macro_rules! Depcrate_options_viewimpl_371 {
() => {
// Module: crate::options::view
// Provides: {"impl_371"}
// Dependencies: {}
impl TerminalWidth { fn deduce < V : Vars > (matches : & MatchedFlags < '_ > , vars : & V) -> Result < Self , OptionsError > { if let Some (width) = matches . get (& flags :: WIDTH) ? { let arg_str = width . to_string_lossy () ; match arg_str . parse () { Ok (w) => { if w >= 1 { Ok (Self :: Set (w)) } else { Ok (Self :: Automatic) } } Err (e) => { let source = NumberSource :: Arg (& flags :: WIDTH) ; Err (OptionsError :: FailedParse (arg_str . to_string () , source , e)) } } } else if let Some (columns) = vars . get (vars :: COLUMNS) . and_then (| s | s . into_string () . ok ()) { match columns . parse () { Ok (width) => Ok (Self :: Set (width)) , Err (e) => { let source = NumberSource :: Env (vars :: COLUMNS) ; Err (OptionsError :: FailedParse (columns , source , e)) } } } else { Ok (Self :: Automatic) } } }
};
}
