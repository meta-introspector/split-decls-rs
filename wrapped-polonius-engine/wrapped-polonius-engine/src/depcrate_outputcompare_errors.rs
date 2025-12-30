// Generated macro for compare_errors (function)
macro_rules! Depcrate_outputcompare_errors {
() => {
// Module: crate::output
// Provides: {"compare_errors"}
// Dependencies: {}
# [doc = " Compares errors reported by Naive implementation with the errors"] # [doc = " reported by the optimized implementation."] fn compare_errors < Loan : Atom , Point : Atom > (all_naive_errors : & FxHashMap < Point , Vec < Loan > > , all_opt_errors : & FxHashMap < Point , Vec < Loan > > ,) -> bool { let points = all_naive_errors . keys () . chain (all_opt_errors . keys ()) ; let mut differ = false ; for point in points { let mut naive_errors = all_naive_errors . get (& point) . cloned () . unwrap_or_default () ; naive_errors . sort () ; let mut opt_errors = all_opt_errors . get (& point) . cloned () . unwrap_or_default () ; opt_errors . sort () ; for err in naive_errors . iter () { if ! opt_errors . contains (err) { error ! ("Error {0:?} at {1:?} reported by naive, but not opt." , err , point) ; differ = true ; } } for err in opt_errors . iter () { if ! naive_errors . contains (err) { error ! ("Error {0:?} at {1:?} reported by opt, but not naive." , err , point) ; differ = true ; } } } differ }
};
}
