// Generated macro for impl_105 (impl)
macro_rules! Depcrate_random_stateimpl_105 {
() => {
// Module: crate::random_state
// Provides: {"impl_105"}
// Dependencies: {}
# [doc = " Creates an instance of RandomState using keys obtained from the random number generator."] # [doc = " Each instance created in this way will have a unique set of keys. (But the resulting instance"] # [doc = " can be used to create many hashers each or which will have the same keys.)"] # [doc = ""] # [doc = " This is the same as [RandomState::new()]"] # [doc = ""] # [doc = " NOTE: For safety this trait impl is only available if either of the flags `runtime-rng` (on by default) or"] # [doc = " `compile-time-rng` are enabled. This is to prevent weakly keyed maps from being accidentally created. Instead one of"] # [doc = " constructors for [RandomState] must be used."] # [cfg (any (feature = "compile-time-rng" , feature = "runtime-rng" , feature = "no-rng"))] impl Default for RandomState { # [inline] fn default () -> Self { Self :: new () } }
};
}
