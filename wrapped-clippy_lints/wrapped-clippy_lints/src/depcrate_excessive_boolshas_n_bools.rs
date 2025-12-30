// Generated macro for has_n_bools (function)
macro_rules! Depcrate_excessive_boolshas_n_bools {
() => {
// Module: crate::excessive_bools
// Provides: {"has_n_bools"}
// Dependencies: {}
fn has_n_bools < 'tcx > (iter : impl Iterator < Item = & 'tcx Ty < 'tcx > > , mut count : u64) -> bool { iter . filter (| ty | is_bool (ty)) . any (| _ | { let (x , overflow) = count . overflowing_sub (1) ; count = x ; overflow }) }
};
}
