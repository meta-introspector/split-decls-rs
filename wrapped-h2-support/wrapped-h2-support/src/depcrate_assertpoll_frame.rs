// Generated macro for poll_frame (macro)
macro_rules! Depcrate_assertpoll_frame {
() => {
// Module: crate::assert
// Provides: {"poll_frame"}
// Dependencies: {}
# [macro_export] macro_rules ! poll_frame { ($ type : ident , $ transport : expr) => { { use futures :: StreamExt ; use h2 :: frame :: Frame ; match $ transport . next () . await { Some (Ok (Frame ::$ type (frame))) => frame , frame => panic ! ("unexpected frame; actual={:?}" , frame) , } } } ; }
};
}
