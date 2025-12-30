// Generated macro for lock (function)
macro_rules! Depcrate_imp_fallbacklock {
() => {
// Module: crate::imp::fallback
// Provides: {"lock"}
// Dependencies: {}
# [inline] # [must_use] fn lock (addr : usize) -> & 'static SeqLock { const LEN : usize = 67 ; const L : CachePadded < SeqLock > = CachePadded :: new (SeqLock :: new ()) ; static LOCKS : [CachePadded < SeqLock > ; LEN] = [L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L , L ,] ; & LOCKS [addr % LEN] }
};
}
