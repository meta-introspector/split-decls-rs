// Generated macro for encode_state (function)
macro_rules! Depcrate_address_channelencode_state {
() => {
// Module: crate::address::channel
// Provides: {"encode_state"}
// Dependencies: {}
fn encode_state (state : & State) -> usize { let mut num = state . num_messages ; if state . is_open { num |= OPEN_MASK ; } num }
};
}
