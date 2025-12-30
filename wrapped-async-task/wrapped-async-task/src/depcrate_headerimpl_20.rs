// Generated macro for impl_20 (impl)
macro_rules! Depcrate_headerimpl_20 {
() => {
// Module: crate::header
// Provides: {"impl_20"}
// Dependencies: {}
impl < M : fmt :: Debug > fmt :: Debug for HeaderWithMetadata < M > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let state = self . header . state . load (Ordering :: SeqCst) ; f . debug_struct ("Header") . field ("scheduled" , & (state & SCHEDULED != 0)) . field ("running" , & (state & RUNNING != 0)) . field ("completed" , & (state & COMPLETED != 0)) . field ("closed" , & (state & CLOSED != 0)) . field ("awaiter" , & (state & AWAITER != 0)) . field ("task" , & (state & TASK != 0)) . field ("ref_count" , & (state / REFERENCE)) . field ("metadata" , & self . metadata) . finish () } }
};
}
