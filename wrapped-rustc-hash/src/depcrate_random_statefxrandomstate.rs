// Generated macro for FxRandomState (struct)
macro_rules! Depcrate_random_stateFxRandomState {
() => {
// Module: crate::random_state
// Provides: {"FxRandomState"}
// Dependencies: {}
# [doc = " `FxRandomState` is an alternative state for `HashMap` types."] # [doc = ""] # [doc = " A particular instance `FxRandomState` will create the same instances of"] # [doc = " [`Hasher`], but the hashers created by two different `FxRandomState`"] # [doc = " instances are unlikely to produce the same result for the same values."] # [derive (Clone)] pub struct FxRandomState { seed : usize , }
};
}
