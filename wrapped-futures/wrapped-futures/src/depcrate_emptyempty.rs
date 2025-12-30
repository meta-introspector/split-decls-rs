// Generated macro for empty (function)
macro_rules! Depcrate_emptyempty {
() => {
// Module: crate::empty
// Provides: {"empty"}
// Dependencies: {}
# [doc = " Creates a future which never resolves, representing a computation that never"] # [doc = " finishes."] # [doc = ""] # [doc = " The returned future will never resolve with a success but is still"] # [doc = " susceptible to cancellation. That is, if a callback is scheduled on the"] # [doc = " returned future, it is only run once the future is dropped (canceled)."] pub fn empty < T : Send + 'static , E : Send + 'static > () -> Empty < T , E > { Empty { _data : marker :: PhantomData } }
};
}
