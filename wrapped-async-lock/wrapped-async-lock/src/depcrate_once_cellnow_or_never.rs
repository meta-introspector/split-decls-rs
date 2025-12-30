// Generated macro for now_or_never (function)
macro_rules! Depcrate_once_cellnow_or_never {
() => {
// Module: crate::once_cell
// Provides: {"now_or_never"}
// Dependencies: {}
# [doc = " Either return the result of a future now, or panic."] # [cfg (all (feature = "std" , not (target_family = "wasm")))] fn now_or_never < T > (f : impl Future < Output = T >) -> T { use core :: pin :: pin ; use core :: task :: { Context , Poll , Waker } ; let f = pin ! (f) ; let mut cx = Context :: from_waker (Waker :: noop ()) ; match f . poll (& mut cx) { Poll :: Ready (value) => value , Poll :: Pending => unreachable ! ("future not ready") , } }
};
}
