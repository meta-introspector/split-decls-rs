// Generated macro for model (function)
macro_rules! Depcrate_modelmodel {
() => {
// Module: crate::model
// Provides: {"model"}
// Dependencies: {}
# [doc = " Run all concurrent permutations of the provided closure."] # [doc = ""] # [doc = " Uses a default [`Builder`] which can be affected by environment variables."] pub fn model < F > (f : F) where F : Fn () + Sync + Send + 'static , { let subscriber = fmt :: Subscriber :: builder () . with_env_filter (EnvFilter :: from_env ("LOOM_LOG")) . with_test_writer () . without_time () . finish () ; subscriber :: with_default (subscriber , | | { Builder :: new () . check (f) ; }) ; }
};
}
