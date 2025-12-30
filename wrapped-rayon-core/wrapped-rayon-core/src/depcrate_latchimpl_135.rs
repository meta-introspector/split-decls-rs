// Generated macro for impl_135 (impl)
macro_rules! Depcrate_latchimpl_135 {
() => {
// Module: crate::latch
// Provides: {"impl_135"}
// Dependencies: {}
impl std :: fmt :: Debug for CountLatchKind { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { CountLatchKind :: Stealing { latch , .. } => { f . debug_tuple ("Stealing") . field (latch) . finish () } CountLatchKind :: Blocking { latch , .. } => { f . debug_tuple ("Blocking") . field (latch) . finish () } } } }
};
}
