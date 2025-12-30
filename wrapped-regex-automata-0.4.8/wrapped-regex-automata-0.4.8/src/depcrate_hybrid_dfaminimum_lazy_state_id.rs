// Generated macro for minimum_lazy_state_id (function)
macro_rules! Depcrate_hybrid_dfaminimum_lazy_state_id {
() => {
// Module: crate::hybrid::dfa
// Provides: {"minimum_lazy_state_id"}
// Dependencies: {}
# [doc = " Based on the minimum number of states required for a useful lazy DFA cache,"] # [doc = " this returns the minimum lazy state ID that must be representable."] # [doc = ""] # [doc = " It's not likely for this to have any impact 32-bit systems (or higher), but"] # [doc = " on 16-bit systems, the lazy state ID space is quite constrained and thus"] # [doc = " may be insufficient if our MIN_STATES value is (for some reason) too high."] fn minimum_lazy_state_id (classes : & ByteClasses ,) -> Result < LazyStateID , LazyStateIDError > { let stride = 1 << classes . stride2 () ; let min_state_index = MIN_STATES . checked_sub (1) . unwrap () ; LazyStateID :: new (min_state_index * stride) }
};
}
