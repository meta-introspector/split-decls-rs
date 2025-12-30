// Generated macro for tests (module)
macro_rules! Depcrate_notetests {
() => {
// Module: crate::note
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [test] fn smoke () { let (_td , repo) = crate :: test :: repo_init () ; assert ! (repo . notes (None) . is_err ()) ; let sig = repo . signature () . unwrap () ; let head = repo . head () . unwrap () . target () . unwrap () ; let note = repo . note (& sig , & sig , None , head , "foo" , false) . unwrap () ; assert_eq ! (repo . notes (None) . unwrap () . count () , 1) ; let note_obj = repo . find_note (None , head) . unwrap () ; assert_eq ! (note_obj . id () , note) ; assert_eq ! (note_obj . message () , Some ("foo")) ; let (a , b) = repo . notes (None) . unwrap () . next () . unwrap () . unwrap () ; assert_eq ! (a , note) ; assert_eq ! (b , head) ; assert_eq ! (repo . note_default_ref () . unwrap () , "refs/notes/commits") ; assert_eq ! (sig . name () , note_obj . author () . name ()) ; assert_eq ! (sig . name () , note_obj . committer () . name ()) ; assert ! (sig . when () == note_obj . committer () . when ()) ; } }
};
}
