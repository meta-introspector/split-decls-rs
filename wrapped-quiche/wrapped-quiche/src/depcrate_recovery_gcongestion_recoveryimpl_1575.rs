// Generated macro for impl_1575 (impl)
macro_rules! Depcrate_recovery_gcongestion_recoveryimpl_1575 {
() => {
// Module: crate::recovery::gcongestion::recovery
// Provides: {"impl_1575"}
// Dependencies: {}
impl SentStatus { fn ack (& mut self) -> Self { std :: mem :: replace (self , SentStatus :: Acked) } fn lose (& mut self) -> Self { if ! matches ! (self , SentStatus :: Acked) { std :: mem :: replace (self , SentStatus :: Lost) } else { SentStatus :: Acked } } }
};
}
