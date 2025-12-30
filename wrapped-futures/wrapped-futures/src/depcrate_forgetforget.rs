// Generated macro for forget (function)
macro_rules! Depcrate_forgetforget {
() => {
// Module: crate::forget
// Provides: {"forget"}
// Dependencies: {}
pub fn forget < T : Future > (t : T) { let thunk = ThunkFuture { inner : t . boxed () } . boxed () ; Task :: new () . run (thunk) }
};
}
