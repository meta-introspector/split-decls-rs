// Generated macro for out (macro)
macro_rules! Depcrate_utils_authorout {
() => {
// Module: crate::utils::author
// Provides: {"out"}
// Dependencies: {}
# [doc = " Writes a line of output with indentation added"] macro_rules ! out { ($ ($ t : tt) *) => { println ! ("    {}" , format_args ! ($ ($ t) *)) } ; }
};
}
