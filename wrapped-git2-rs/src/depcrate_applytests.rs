// Generated macro for tests (module)
macro_rules! Depcrate_applytests {
() => {
// Module: crate::apply
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use std :: { fs :: File , io :: Write , path :: Path } ; # [test] fn smoke_test () { let (_td , repo) = crate :: test :: repo_init () ; let diff = t ! (repo . diff_tree_to_workdir (None , None)) ; let mut count_hunks = 0 ; let mut count_delta = 0 ; { let mut opts = ApplyOptions :: new () ; opts . hunk_callback (| _hunk | { count_hunks += 1 ; true }) ; opts . delta_callback (| _delta | { count_delta += 1 ; true }) ; t ! (repo . apply (& diff , ApplyLocation :: Both , Some (& mut opts))) ; } assert_eq ! (count_hunks , 0) ; assert_eq ! (count_delta , 0) ; } # [test] fn apply_hunks_and_delta () { let file_path = Path :: new ("foo.txt") ; let (td , repo) = crate :: test :: repo_init () ; t ! (t ! (File :: create (& td . path () . join (file_path))) . write_all (b"bar")) ; t ! (t ! (repo . index ()) . add_path (file_path)) ; t ! (t ! (File :: create (& td . path () . join (file_path))) . write_all (b"foo\nbar")) ; let diff = t ! (repo . diff_index_to_workdir (None , None)) ; assert_eq ! (diff . deltas () . len () , 1) ; let mut count_hunks = 0 ; let mut count_delta = 0 ; { let mut opts = ApplyOptions :: new () ; opts . hunk_callback (| _hunk | { count_hunks += 1 ; true }) ; opts . delta_callback (| _delta | { count_delta += 1 ; true }) ; t ! (repo . apply (& diff , ApplyLocation :: Index , Some (& mut opts))) ; } assert_eq ! (count_delta , 1) ; assert_eq ! (count_hunks , 1) ; } }
};
}
