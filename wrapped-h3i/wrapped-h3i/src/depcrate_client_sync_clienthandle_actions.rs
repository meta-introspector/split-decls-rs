// Generated macro for handle_actions (function)
macro_rules! Depcrate_client_sync_clienthandle_actions {
() => {
// Module: crate::client::sync_client
// Provides: {"handle_actions"}
// Dependencies: {}
fn handle_actions < 'a , I > (iter : & mut I , conn : & mut quiche :: Connection , waiting_for : & mut WaitingFor , stream_parsers : & mut StreamParserMap ,) -> Option < Duration > where I : Iterator < Item = & 'a Action > , { if ! waiting_for . is_empty () { log :: debug ! ("won't fire an action due to waiting for responses: {waiting_for:?}") ; return None ; } for action in iter { match action { Action :: FlushPackets => return None , Action :: Wait { wait_type } => match wait_type { WaitType :: WaitDuration (period) => return Some (* period) , WaitType :: StreamEvent (response) => { log :: info ! ("waiting for {response:?} before executing more actions") ; waiting_for . add_wait (response) ; return None ; } , } , action => execute_action (action , conn , stream_parsers) , } } None }
};
}
