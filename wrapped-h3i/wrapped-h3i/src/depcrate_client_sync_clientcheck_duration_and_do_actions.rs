// Generated macro for check_duration_and_do_actions (function)
macro_rules! Depcrate_client_sync_clientcheck_duration_and_do_actions {
() => {
// Module: crate::client::sync_client
// Provides: {"check_duration_and_do_actions"}
// Dependencies: {}
fn check_duration_and_do_actions (wait_duration : & mut Option < Duration > , wait_instant : & mut Option < Instant > , action_iter : & mut Iter < Action > , conn : & mut quiche :: Connection , waiting_for : & mut WaitingFor , stream_parsers : & mut StreamParserMap ,) { match wait_duration . as_ref () { None => { if let Some (idle_wait) = handle_actions (action_iter , conn , waiting_for , stream_parsers) { * wait_duration = Some (idle_wait) ; * wait_instant = Some (Instant :: now ()) ; log :: info ! ("waiting for {idle_wait:?} before executing more actions") ; } } , Some (period) => { let now = Instant :: now () ; let then = wait_instant . unwrap () ; log :: debug ! ("checking if actions wait period elapsed {:?} > {:?}" , now . duration_since (then) , wait_duration) ; if now . duration_since (then) >= * period { log :: debug ! ("yup!") ; * wait_duration = None ; if let Some (idle_wait) = handle_actions (action_iter , conn , waiting_for , stream_parsers) { * wait_duration = Some (idle_wait) ; } } } , } }
};
}
