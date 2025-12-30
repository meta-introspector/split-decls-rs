// Generated macro for check_only_timeout (function)
macro_rules! Depcrate_tests_register_bufferscheck_only_timeout {
() => {
// Module: crate::tests::register_buffers
// Provides: {"check_only_timeout"}
// Dependencies: {}
# [doc = " Submit sqes and asserts the only cqe is a timeout entry"] fn check_only_timeout < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > ,) -> Result < () , anyhow :: Error > { ring . submit_and_wait (1) ? ; if Into :: < cqueue :: Entry > :: into (ring . completion () . next () . unwrap ()) . user_data () == TIMEOUT_TAG { if ring . completion () . next () . is_none () { return Ok (()) ; } } Err (anyhow :: anyhow ! ("unexpected completion queue entry")) }
};
}
