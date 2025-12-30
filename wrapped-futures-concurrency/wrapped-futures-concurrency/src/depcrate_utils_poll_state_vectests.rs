// Generated macro for tests (module)
macro_rules! Depcrate_utils_poll_state_vectests {
() => {
// Module: crate::utils::poll_state::vec
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { PollVec , MAX_INLINE_ENTRIES } ; # [test] fn type_size () { assert_eq ! (core :: mem :: size_of ::< PollVec > () , core :: mem :: size_of ::< usize > () * 4) ; } # [test] fn boxed_does_not_allocate_twice () { let _ = PollVec :: new_pending (MAX_INLINE_ENTRIES + 10) ; } }
};
}
