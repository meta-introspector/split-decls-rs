// Generated macro for check_hir_nesting (function)
macro_rules! Depcrate_hir_parsecheck_hir_nesting {
() => {
// Module: crate::hir::parse
// Provides: {"check_hir_nesting"}
// Dependencies: {}
# [doc = " This checks the depth of the given `Hir` value, and if it exceeds the given"] # [doc = " limit, then an error is returned."] fn check_hir_nesting (hir : & Hir , limit : u32) -> Result < () , Error > { fn recurse (hir : & Hir , limit : u32 , depth : u32) -> Result < () , Error > { if depth > limit { return Err (Error :: new (ERR_TOO_MUCH_NESTING)) ; } let Some (next_depth) = depth . checked_add (1) else { return Err (Error :: new (ERR_TOO_MUCH_NESTING)) ; } ; match * hir . kind () { HirKind :: Empty | HirKind :: Char (_) | HirKind :: Class (_) | HirKind :: Look (_) => Ok (()) , HirKind :: Repetition (hir :: Repetition { ref sub , .. }) => { recurse (sub , limit , next_depth) } HirKind :: Capture (hir :: Capture { ref sub , .. }) => { recurse (sub , limit , next_depth) } HirKind :: Concat (ref subs) | HirKind :: Alternation (ref subs) => { for sub in subs . iter () { recurse (sub , limit , next_depth) ? ; } Ok (()) } } } recurse (hir , limit , 0) }
};
}
