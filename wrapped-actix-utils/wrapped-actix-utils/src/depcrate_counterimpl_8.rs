// Generated macro for impl_8 (impl)
macro_rules! Depcrate_counterimpl_8 {
() => {
// Module: crate::counter
// Provides: {"impl_8"}
// Dependencies: {}
impl CounterInner { fn inc (& self) { self . count . set (self . count . get () + 1) ; } fn dec (& self) { let num = self . count . get () ; self . count . set (num - 1) ; if num == self . capacity { self . task . wake () ; } } fn available (& self , cx : & task :: Context < '_ >) -> bool { if self . count . get () < self . capacity { true } else { self . task . register (cx . waker ()) ; false } } }
};
}
