// Generated macro for block_on (function)
macro_rules! Depcrate_rt_async_supportblock_on {
() => {
// Module: crate::rt::async_support
// Provides: {"block_on"}
// Dependencies: {}
# [doc = " Run the specified future to completion, returning the result."] # [doc = ""] # [doc = " This uses `waitable-set.wait` to poll for progress on any in-progress calls"] # [doc = " to async-lowered imports as necessary."] pub fn block_on < T : 'static > (future : impl Future < Output = T >) -> T { let mut result = None ; let mut state = FutureState :: new (Box :: pin (async { result = Some (future . await) ; })) ; let mut event = (EVENT_NONE , 0 , 0) ; loop { match state . callback (event . 0 , event . 1 , event . 2) { (_ , true) => { drop (state) ; break result . unwrap () ; } (CALLBACK_CODE_YIELD , false) => event = state . waitable_set . as_ref () . unwrap () . poll () , _ => event = state . waitable_set . as_ref () . unwrap () . wait () , } } }
};
}
