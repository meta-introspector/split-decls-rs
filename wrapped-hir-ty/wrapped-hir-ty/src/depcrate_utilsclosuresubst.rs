// Generated macro for ClosureSubst (struct)
macro_rules! Depcrate_utilsClosureSubst {
() => {
// Module: crate::utils
// Provides: {"ClosureSubst"}
// Dependencies: {}
# [doc = " It is a bit different from the rustc equivalent. Currently it stores:"] # [doc = " - 0..n-1: generics of the parent"] # [doc = " - n: the function signature, encoded as a function pointer type"] # [doc = ""] # [doc = " and it doesn't store the closure types and fields."] # [doc = ""] # [doc = " Codes should not assume this ordering, and should always use methods available"] # [doc = " on this struct for retrieving, and `TyBuilder::substs_for_closure` for creating."] pub (crate) struct ClosureSubst < 'a > (pub (crate) & 'a Substitution) ;
};
}
