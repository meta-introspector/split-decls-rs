// Generated macro for PoolInner (struct)
macro_rules! Depcrate_client_legacy_poolPoolInner {
() => {
// Module: crate::client::legacy::pool
// Provides: {"PoolInner"}
// Dependencies: {}
# [doc = " Simple type alias in case the key type needs to be adjusted."] struct PoolInner < T , K : Eq + Hash > { connecting : HashSet < K > , idle : HashMap < K , Vec < Idle < T > > > , max_idle_per_host : usize , waiters : HashMap < K , VecDeque < oneshot :: Sender < T > > > , idle_interval_ref : Option < oneshot :: Sender < Infallible > > , exec : Exec , timer : Option < Timer > , timeout : Option < Duration > , }
};
}
