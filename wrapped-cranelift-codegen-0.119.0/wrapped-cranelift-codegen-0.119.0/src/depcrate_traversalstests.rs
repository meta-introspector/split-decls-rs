// Generated macro for tests (module)
macro_rules! Depcrate_traversalstests {
() => {
// Module: crate::traversals
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: cursor :: { Cursor , FuncCursor } ; use crate :: ir :: { types :: I32 , Function , InstBuilder , TrapCode } ; # [test] fn test_dfs_traversal () { let _ = env_logger :: try_init () ; let mut func = Function :: new () ; let block0 = func . dfg . make_block () ; let v0 = func . dfg . append_block_param (block0 , I32) ; let block1 = func . dfg . make_block () ; let block2 = func . dfg . make_block () ; let block3 = func . dfg . make_block () ; let mut cur = FuncCursor :: new (& mut func) ; cur . insert_block (block0) ; cur . ins () . brif (v0 , block2 , & [] , block3 , & []) ; cur . insert_block (block3) ; cur . ins () . trap (TrapCode :: unwrap_user (1)) ; cur . insert_block (block1) ; let v1 = cur . ins () . iconst (I32 , 1) ; let v2 = cur . ins () . iadd (v0 , v1) ; cur . ins () . jump (block0 , & [v2]) ; cur . insert_block (block2) ; cur . ins () . return_ (& [v0]) ; let mut dfs = Dfs :: new () ; assert_eq ! (dfs . iter (& func) . collect ::< Vec < _ >> () , vec ! [(Event :: Enter , block0) , (Event :: Enter , block2) , (Event :: Exit , block2) , (Event :: Enter , block3) , (Event :: Exit , block3) , (Event :: Exit , block0)] ,) ; } }
};
}
