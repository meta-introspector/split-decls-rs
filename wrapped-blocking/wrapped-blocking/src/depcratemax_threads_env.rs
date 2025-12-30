// Generated macro for MAX_THREADS_ENV (const)
macro_rules! DepcrateMAX_THREADS_ENV {
() => {
// Module: crate
// Provides: {"MAX_THREADS_ENV"}
// Dependencies: {}
# [doc = " Env variable that allows to override default value for max threads."] # [cfg (not (target_family = "wasm"))] const MAX_THREADS_ENV : & str = "BLOCKING_MAX_THREADS" ;
};
}
