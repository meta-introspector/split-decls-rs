// Generated macro for impl_1039 (impl)
macro_rules! Depcrate_recoveryimpl_1039 {
() => {
// Module: crate::recovery
// Provides: {"impl_1039"}
// Dependencies: {}
impl std :: fmt :: Debug for LossDetectionTimer { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { match self . time { Some (v) => { let now = Instant :: now () ; if v > now { let d = v . duration_since (now) ; write ! (f , "{d:?}") } else { write ! (f , "exp") } } , None => write ! (f , "none") , } } }
};
}
