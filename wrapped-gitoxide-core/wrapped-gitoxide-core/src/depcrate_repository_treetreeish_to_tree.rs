// Generated macro for treeish_to_tree (function)
macro_rules! Depcrate_repository_treetreeish_to_tree {
() => {
// Module: crate::repository::tree
// Provides: {"treeish_to_tree"}
// Dependencies: {}
fn treeish_to_tree < 'repo > (treeish : Option < & str > , repo : & 'repo gix :: Repository) -> anyhow :: Result < Tree < 'repo > > { let spec = treeish . map_or_else (| | "@^{tree}" . into () , | spec | format ! ("{spec}^{{tree}}")) ; Ok (repo . rev_parse_single (spec . as_str ()) ? . object () ? . into_tree ()) }
};
}
