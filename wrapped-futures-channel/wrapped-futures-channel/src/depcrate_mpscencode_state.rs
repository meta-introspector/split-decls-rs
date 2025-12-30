// Generated macro for encode_state (function)
macro_rules! Depcrate_mpscencode_state {
() => {
// Module: crate::mpsc
// Provides: {"encode_state"}
// Dependencies: {}
fn encode_state (state : & State) -> usize { let mut num = state . num_messages ; if state . is_open { num |= OPEN_MASK ; } num }
};
}
