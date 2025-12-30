// Generated macro for resolve (function)
macro_rules! Depcrate_file_includesresolve {
() => {
// Module: crate::file::includes
// Provides: {"resolve"}
// Dependencies: {}
pub (crate) fn resolve (config : & mut File < 'static > , buf : & mut Vec < u8 > , options : init :: Options < '_ >) -> Result < () , Error > { resolve_includes_recursive (None , config , 0 , buf , options) }
};
}
