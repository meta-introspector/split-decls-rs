// Generated macro for tests (module)
macro_rules! Depcrate_blametests {
() => {
// Module: crate::blame
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: fs :: { self , File } ; use std :: path :: Path ; # [test] fn smoke () { let (_td , repo) = crate :: test :: repo_init () ; let mut index = repo . index () . unwrap () ; let root = repo . workdir () . unwrap () ; fs :: create_dir (& root . join ("foo")) . unwrap () ; File :: create (& root . join ("foo/bar")) . unwrap () ; index . add_path (Path :: new ("foo/bar")) . unwrap () ; let id = index . write_tree () . unwrap () ; let tree = repo . find_tree (id) . unwrap () ; let sig = repo . signature () . unwrap () ; let id = repo . refname_to_id ("HEAD") . unwrap () ; let parent = repo . find_commit (id) . unwrap () ; let commit = repo . commit (Some ("HEAD") , & sig , & sig , "commit" , & tree , & [& parent]) . unwrap () ; let blame = repo . blame_file (Path :: new ("foo/bar") , None) . unwrap () ; assert_eq ! (blame . len () , 1) ; assert_eq ! (blame . iter () . count () , 1) ; let hunk = blame . get_index (0) . unwrap () ; assert_eq ! (hunk . final_commit_id () , commit) ; assert_eq ! (hunk . final_signature () . name () , sig . name ()) ; assert_eq ! (hunk . final_signature () . email () , sig . email ()) ; assert_eq ! (hunk . final_start_line () , 1) ; assert_eq ! (hunk . path () , Some (Path :: new ("foo/bar"))) ; assert_eq ! (hunk . lines_in_hunk () , 0) ; assert ! (! hunk . is_boundary ()) ; let blame_buffer = blame . blame_buffer ("\n" . as_bytes ()) . unwrap () ; let line = blame_buffer . get_line (1) . unwrap () ; assert_eq ! (blame_buffer . len () , 2) ; assert_eq ! (blame_buffer . iter () . count () , 2) ; assert ! (line . final_commit_id () . is_zero ()) ; } }
};
}
