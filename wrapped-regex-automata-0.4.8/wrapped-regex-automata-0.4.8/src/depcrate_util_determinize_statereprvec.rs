// Generated macro for ReprVec (struct)
macro_rules! Depcrate_util_determinize_stateReprVec {
() => {
// Module: crate::util::determinize::state
// Provides: {"ReprVec"}
// Dependencies: {}
# [doc = " ReprVec is a write-only view into the representation of a DFA state."] # [doc = ""] # [doc = " See Repr for more details on the purpose of this type and also the format."] # [doc = ""] # [doc = " Note that not all possible combinations of methods may be called. This is"] # [doc = " precisely what the various StateBuilder types encapsulate: they only"] # [doc = " permit valid combinations via Rust's linear typing."] struct ReprVec < 'a > (& 'a mut Vec < u8 >) ;
};
}
