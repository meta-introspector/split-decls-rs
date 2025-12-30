// Generated macro for UnknownMismatch (struct)
macro_rules! Depcrate_inferUnknownMismatch {
() => {
// Module: crate::infer
// Provides: {"UnknownMismatch"}
// Dependencies: {}
# [doc = " A zipper that checks for unequal occurrences of `{unknown}` and unresolved projections"] # [doc = " in the two types. Used to filter out mismatch diagnostics that only differ in"] # [doc = " `{unknown}` and unresolved projections. These mismatches are usually not helpful."] # [doc = " As the cause is usually an underlying name resolution problem"] struct UnknownMismatch < 'db > (& 'db dyn HirDatabase) ;
};
}
