// Generated macro for nth_arg (function)
macro_rules! Depcratenth_arg {
() => {
// Module: crate
// Provides: {"nth_arg"}
// Dependencies: {}
# [doc = " Convenience function to get the nth argument type of a function."] pub fn nth_arg < 'tcx > (cx : & LateContext < 'tcx > , fn_def_id : OwnerId , nth : usize) -> Ty < 'tcx > { let arg = cx . tcx . fn_sig (fn_def_id) . instantiate_identity () . input (nth) ; cx . tcx . instantiate_bound_regions_with_erased (arg) }
};
}
