// Generated macro for commit (function)
macro_rules! Depcrate_gitcommit {
() => {
// Module: crate::git
// Provides: {"commit"}
// Dependencies: {}
# [doc = " *(`git2`)* Commit changes to the git repository"] pub fn commit (repo : & git2 :: Repository) -> git2 :: Oid { let tree_id = t ! (t ! (repo . index ()) . write_tree ()) ; let sig = t ! (repo . signature ()) ; let mut parents = Vec :: new () ; if let Some (parent) = repo . head () . ok () . map (| h | h . target () . unwrap ()) { parents . push (t ! (repo . find_commit (parent))) } let parents = parents . iter () . collect :: < Vec < _ > > () ; t ! (repo . commit (Some ("HEAD") , & sig , & sig , "test" , & t ! (repo . find_tree (tree_id)) , & parents)) }
};
}
