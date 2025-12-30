// Generated macro for decode_state (function)
macro_rules! Depcrate_address_channeldecode_state {
() => {
// Module: crate::address::channel
// Provides: {"decode_state"}
// Dependencies: {}
fn decode_state (num : usize) -> State { State { is_open : num & OPEN_MASK == OPEN_MASK , num_messages : num & MAX_CAPACITY , } }
};
}
