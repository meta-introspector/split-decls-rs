// Generated macro for commit (function)
macro_rules! Depcrate_testcommit {
() => {
// Module: crate::test
// Provides: {"commit"}
// Dependencies: {}
pub fn commit (repo : & Repository) -> (Oid , Oid) { let mut index = t ! (repo . index ()) ; let root = repo . path () . parent () . unwrap () ; t ! (File :: create (& root . join ("foo"))) ; t ! (index . add_path (Path :: new ("foo"))) ; let tree_id = t ! (index . write_tree ()) ; let tree = t ! (repo . find_tree (tree_id)) ; let sig = t ! (repo . signature ()) ; let head_id = t ! (repo . refname_to_id ("HEAD")) ; let parent = t ! (repo . find_commit (head_id)) ; let commit = t ! (repo . commit (Some ("HEAD") , & sig , & sig , "commit" , & tree , & [& parent])) ; (commit , tree_id) }
};
}
