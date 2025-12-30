// Generated macro for validate_vis (function)
macro_rules! Depcrate_systemvalidate_vis {
() => {
// Module: crate::system
// Provides: {"validate_vis"}
// Dependencies: {}
fn validate_vis (vis : & Visibility) -> Result < () > { if ! matches ! (vis , Visibility :: Public (_)) { bail ! (vis , "#[system] functions must be public") ; } Ok (()) }
};
}
