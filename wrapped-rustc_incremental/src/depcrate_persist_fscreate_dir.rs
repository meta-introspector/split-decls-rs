// Generated macro for create_dir (function)
macro_rules! Depcrate_persist_fscreate_dir {
() => {
// Module: crate::persist::fs
// Provides: {"create_dir"}
// Dependencies: {}
fn create_dir (sess : & Session , path : & Path , dir_tag : & str) { match std_fs :: create_dir_all (path) { Ok (()) => { debug ! ("{} directory created successfully" , dir_tag) ; } Err (err) => sess . dcx () . emit_fatal (errors :: CreateIncrCompDir { tag : dir_tag , path , err }) , } }
};
}
