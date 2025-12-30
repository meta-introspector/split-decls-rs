// Generated macro for decode_state (function)
macro_rules! Depcrate_mpscdecode_state {
() => {
// Module: crate::mpsc
// Provides: {"decode_state"}
// Dependencies: {}
fn decode_state (num : usize) -> State { State { is_open : num & OPEN_MASK == OPEN_MASK , num_messages : num & MAX_CAPACITY } }
};
}
