// Generated macro for poll_err (macro)
macro_rules! Depcrate_assertpoll_err {
() => {
// Module: crate::assert
// Provides: {"poll_err"}
// Dependencies: {}
# [macro_export] macro_rules ! poll_err { ($ transport : expr) => { { use futures :: StreamExt ; match $ transport . next () . await { Some (Err (e)) => e , frame => panic ! ("expected error; actual={:?}" , frame) , } } } ; }
};
}
