// Generated macro for State (struct)
macro_rules! Depcrate_dfaState {
() => {
// Module: crate::dfa
// Provides: {"State"}
// Dependencies: {}
# [doc = " State is a DFA state. It contains an ordered set of NFA states (not"] # [doc = " necessarily complete) and a smattering of flags."] # [doc = ""] # [doc = " The flags are packed into the first byte of data."] # [doc = ""] # [doc = " States don't carry their transitions. Instead, transitions are stored in"] # [doc = " a single row-major table."] # [doc = ""] # [doc = " Delta encoding is used to store the instruction pointers."] # [doc = " The first instruction pointer is stored directly starting"] # [doc = " at data[1], and each following pointer is stored as an offset"] # [doc = " to the previous one. If a delta is in the range -127..127,"] # [doc = " it is packed into a single byte; Otherwise the byte 128 (-128 as an i8)"] # [doc = " is coded as a flag, followed by 4 bytes encoding the delta."] # [derive (Clone , Eq , Hash , PartialEq)] struct State { data : Box < [u8] > , }
};
}
