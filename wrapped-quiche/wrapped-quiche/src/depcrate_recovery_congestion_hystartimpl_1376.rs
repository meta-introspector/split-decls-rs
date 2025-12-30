// Generated macro for impl_1376 (impl)
macro_rules! Depcrate_recovery_congestion_hystartimpl_1376 {
() => {
// Module: crate::recovery::congestion::hystart
// Provides: {"impl_1376"}
// Dependencies: {}
impl std :: fmt :: Debug for Hystart { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { write ! (f , "window_end={:?} " , self . window_end) ? ; write ! (f , "last_round_min_rtt={:?} " , self . last_round_min_rtt) ? ; write ! (f , "current_round_min_rtt={:?} " , self . current_round_min_rtt) ? ; write ! (f , "css_baseline_min_rtt={:?} " , self . css_baseline_min_rtt) ? ; write ! (f , "rtt_sample_count={:?} " , self . rtt_sample_count) ? ; write ! (f , "css_start_time={:?} " , self . css_start_time) ? ; write ! (f , "css_round_count={:?}" , self . css_round_count) ? ; Ok (()) } }
};
}
