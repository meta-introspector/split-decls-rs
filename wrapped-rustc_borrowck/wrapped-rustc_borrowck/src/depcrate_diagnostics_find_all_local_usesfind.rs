// Generated macro for find (function)
macro_rules! Depcrate_diagnostics_find_all_local_usesfind {
() => {
// Module: crate::diagnostics::find_all_local_uses
// Provides: {"find"}
// Dependencies: {}
# [doc = " Find all uses of (including assignments to) a [`Local`]."] # [doc = ""] # [doc = " Uses `BTreeSet` so output is deterministic."] pub (super) fn find (body : & Body < '_ > , local : Local) -> BTreeSet < Location > { let mut visitor = AllLocalUsesVisitor { for_local : local , uses : BTreeSet :: default () } ; visitor . visit_body (body) ; visitor . uses }
};
}
