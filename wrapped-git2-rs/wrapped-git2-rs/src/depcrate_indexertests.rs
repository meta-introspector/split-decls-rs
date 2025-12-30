// Generated macro for tests (module)
macro_rules! Depcrate_indexertests {
() => {
// Module: crate::indexer
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: { Buf , Indexer } ; use std :: io :: prelude :: * ; # [test] fn indexer () { let (_td , repo_source) = crate :: test :: repo_init () ; let (_td , repo_target) = crate :: test :: repo_init () ; let mut progress_called = false ; let mut builder = t ! (repo_source . packbuilder ()) ; let mut buf = Buf :: new () ; let (commit_source_id , _tree) = crate :: test :: commit (& repo_source) ; t ! (builder . insert_object (commit_source_id , None)) ; t ! (builder . write_buf (& mut buf)) ; let odb = repo_source . odb () . unwrap () ; let mut indexer = Indexer :: new (Some (& odb) , repo_target . path () . join ("objects") . join ("pack") . as_path () , 0o644 , true ,) . unwrap () ; indexer . progress (| _ | { progress_called = true ; true }) ; indexer . write (& buf) . unwrap () ; indexer . commit () . unwrap () ; let commit_target = repo_target . find_commit (commit_source_id) . unwrap () ; assert_eq ! (commit_target . id () , commit_source_id) ; assert ! (progress_called) ; } }
};
}
