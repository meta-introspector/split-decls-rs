// Generated macro for impl_1431 (impl)
macro_rules! Depcrate_recovery_congestion_recoveryimpl_1431 {
() => {
// Module: crate::recovery::congestion::recovery
// Provides: {"impl_1431"}
// Dependencies: {}
impl std :: fmt :: Debug for LegacyRecovery { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { write ! (f , "timer={:?} " , self . loss_timer) ? ; write ! (f , "latest_rtt={:?} " , self . rtt_stats . latest_rtt) ? ; write ! (f , "srtt={:?} " , self . rtt_stats . smoothed_rtt) ? ; write ! (f , "min_rtt={:?} " , * self . rtt_stats . min_rtt) ? ; write ! (f , "rttvar={:?} " , self . rtt_stats . rttvar) ? ; write ! (f , "cwnd={} " , self . cwnd ()) ? ; write ! (f , "ssthresh={} " , self . congestion . ssthresh . get ()) ? ; write ! (f , "bytes_in_flight={} " , self . bytes_in_flight . get ()) ? ; write ! (f , "app_limited={} " , self . congestion . app_limited) ? ; write ! (f , "congestion_recovery_start_time={:?} " , self . congestion . congestion_recovery_start_time) ? ; write ! (f , "{:?} " , self . congestion . delivery_rate) ? ; write ! (f , "pacer={:?} " , self . congestion . pacer) ? ; if self . congestion . hystart . enabled () { write ! (f , "hystart={:?} " , self . congestion . hystart) ? ; } (self . congestion . cc_ops . debug_fmt) (& self . congestion , f) ? ; Ok (()) } }
};
}
