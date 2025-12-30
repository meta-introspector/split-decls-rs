// Generated macro for iter_input_pats (function)
macro_rules! Depcrateiter_input_pats {
() => {
// Module: crate
// Provides: {"iter_input_pats"}
// Dependencies: {}
pub fn iter_input_pats < 'tcx > (decl : & FnDecl < '_ > , body : & 'tcx Body < '_ >) -> impl Iterator < Item = & 'tcx Param < 'tcx > > { (0 .. decl . inputs . len ()) . map (move | i | & body . params [i]) }
};
}
