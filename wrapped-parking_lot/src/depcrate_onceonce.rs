// Generated macro for Once (struct)
macro_rules! Depcrate_onceOnce {
() => {
// Module: crate::once
// Provides: {"Once"}
// Dependencies: {}
# [doc = " A synchronization primitive which can be used to run a one-time"] # [doc = " initialization. Useful for one-time initialization for globals, FFI or"] # [doc = " related functionality."] # [doc = ""] # [doc = " # Differences from the standard library `Once`"] # [doc = ""] # [doc = " - Only requires 1 byte of space, instead of 1 word."] # [doc = " - Not required to be `'static`."] # [doc = " - Relaxed memory barriers in the fast path, which can significantly improve"] # [doc = "   performance on some architectures."] # [doc = " - Efficient handling of micro-contention using adaptive spinning."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use parking_lot::Once;"] # [doc = ""] # [doc = " static START: Once = Once::new();"] # [doc = ""] # [doc = " START.call_once(|| {"] # [doc = "     // run initialization here"] # [doc = " });"] # [doc = " ```"] pub struct Once (AtomicU8) ;
};
}
