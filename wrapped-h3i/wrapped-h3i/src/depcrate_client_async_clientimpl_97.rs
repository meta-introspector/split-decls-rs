// Generated macro for impl_97 (impl)
macro_rules! Depcrate_client_async_clientimpl_97 {
() => {
// Module: crate::client::async_client
// Provides: {"impl_97"}
// Dependencies: {}
impl H3iDriver { fn new (actions : Vec < Action > , close_trigger_frames : Option < CloseTriggerFrames > ,) -> (Self , BuildingConnectionSummary) { let (record_tx , record_rx) = mpsc :: unbounded_channel () ; let (close_trigger_seen_tx , close_trigger_seen_rx) = oneshot :: channel () ; let fut = BuildingConnectionSummary :: new (record_rx , close_trigger_frames , close_trigger_seen_tx ,) ; (Self { buffer : BufFactory :: get_max_buf () , actions , actions_executed : 0 , next_fire_time : Instant :: now () , waiting_for_responses : WaitingFor :: default () , record_tx , stream_parsers : StreamParserMap :: default () , close_trigger_seen_rx , } , fut ,) } # [doc = " If the next action should fire."] fn should_fire (& self) -> bool { Instant :: now () >= self . next_fire_time } # [doc = " Insert all waits into the waiting set."] fn register_waits (& mut self) { while self . actions_executed < self . actions . len () { if let Action :: Wait { wait_type } = & self . actions [self . actions_executed] { self . actions_executed += 1 ; match wait_type { WaitType :: WaitDuration (duration) => { self . next_fire_time = Instant :: now () + * duration ; log :: debug ! ("h3i: waiting for responses: {:?}" , self . waiting_for_responses) ; } , WaitType :: StreamEvent (event) => { self . waiting_for_responses . add_wait (event) ; } , } } else { break ; } } } }
};
}
