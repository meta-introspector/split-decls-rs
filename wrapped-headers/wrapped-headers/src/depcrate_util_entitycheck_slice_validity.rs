// Generated macro for check_slice_validity (function)
macro_rules! Depcrate_util_entitycheck_slice_validity {
() => {
// Module: crate::util::entity
// Provides: {"check_slice_validity"}
// Dependencies: {}
# [doc = " check that each char in the slice is either:"] # [doc = " 1. `%x21`, or"] # [doc = " 2. in the range `%x23` to `%x7E`, or"] # [doc = " 3. above `%x80`"] fn check_slice_validity (slice : & [u8]) -> bool { slice . iter () . all (| & c | { debug_assert ! ((b'\x21' ..= b'\x7e') . contains (& c) | (c >= b'\x80') , "EntityTag expects HeaderValue to have check for control characters") ; c != b'"' }) }
};
}
