// Generated macro for NeedlessBorrowsForGenericArgs (struct)
macro_rules! Depcrate_needless_borrows_for_generic_argsNeedlessBorrowsForGenericArgs {
() => {
// Module: crate::needless_borrows_for_generic_args
// Provides: {"NeedlessBorrowsForGenericArgs"}
// Dependencies: {}
pub struct NeedlessBorrowsForGenericArgs < 'tcx > { # [doc = " Stack of (body owner, `PossibleBorrowerMap`) pairs. Used by"] # [doc = " [`needless_borrow_count`] to determine when a borrowed expression can instead"] # [doc = " be moved."] possible_borrowers : Vec < (LocalDefId , PossibleBorrowerMap < 'tcx , 'tcx >) > , msrv : Msrv , }
};
}
