// Generated macro for count_needed_derefs (function)
macro_rules! Depcrate_format_argscount_needed_derefs {
() => {
// Module: crate::format_args
// Provides: {"count_needed_derefs"}
// Dependencies: {}
fn count_needed_derefs < 'tcx , I > (mut ty : Ty < 'tcx > , mut iter : I) -> (usize , Ty < 'tcx >) where I : Iterator < Item = & 'tcx Adjustment < 'tcx > > , { let mut n_total = 0 ; let mut n_needed = 0 ; loop { if let Some (Adjustment { kind : Adjust :: Deref (overloaded_deref) , target , }) = iter . next () { n_total += 1 ; if overloaded_deref . is_some () { n_needed = n_total ; } ty = * target ; } else { return (n_needed , ty) ; } } }
};
}
