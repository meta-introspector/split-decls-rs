// Generated macro for impl_26 (impl)
macro_rules! Depcrate_cleanimpl_26 {
() => {
// Module: crate::clean
// Provides: {"impl_26"}
// Dependencies: {}
impl CleanArg { fn new () -> Result < Self , String > { if let Some (arg) = std :: env :: args () . nth (2) { return match arg . as_str () { "all" => Ok (Self :: All) , "ui-tests" => Ok (Self :: UiTests) , "--help" => Ok (Self :: Help) , a => Err (format ! ("Unknown argument `{a}`")) , } ; } Ok (Self :: default ()) } }
};
}
