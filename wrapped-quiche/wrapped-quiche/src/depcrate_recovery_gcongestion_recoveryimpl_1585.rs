// Generated macro for impl_1585 (impl)
macro_rules! Depcrate_recovery_gcongestion_recoveryimpl_1585 {
() => {
// Module: crate::recovery::gcongestion::recovery
// Provides: {"impl_1585"}
// Dependencies: {}
impl std :: fmt :: Debug for GRecovery { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { write ! (f , "timer={:?} " , self . loss_detection_timer ()) ? ; write ! (f , "rtt_stats={:?} " , self . rtt_stats) ? ; write ! (f , "bytes_in_flight={} " , self . bytes_in_flight . get ()) ? ; write ! (f , "{:?} " , self . pacer) ? ; Ok (()) } }
};
}
