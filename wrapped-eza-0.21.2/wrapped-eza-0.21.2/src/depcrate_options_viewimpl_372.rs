// Generated macro for impl_372 (impl)
macro_rules! Depcrate_options_viewimpl_372 {
() => {
// Module: crate::options::view
// Provides: {"impl_372"}
// Dependencies: {}
impl RowThreshold { fn deduce < V : Vars > (vars : & V) -> Result < Self , OptionsError > { if let Some (columns) = vars . get_with_fallback (vars :: EZA_GRID_ROWS , vars :: EXA_GRID_ROWS) . and_then (| s | s . into_string () . ok ()) { match columns . parse () { Ok (rows) => Ok (Self :: MinimumRows (rows)) , Err (e) => { let source = NumberSource :: Env (vars . source (vars :: EZA_GRID_ROWS , vars :: EXA_GRID_ROWS) . unwrap () ,) ; Err (OptionsError :: FailedParse (columns , source , e)) } } } else { Ok (Self :: AlwaysGrid) } } }
};
}
