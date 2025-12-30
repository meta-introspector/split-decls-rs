// Generated macro for impl_1581 (impl)
macro_rules! Depcrate_recovery_gcongestion_recoveryimpl_1581 {
() => {
// Module: crate::recovery::gcongestion::recovery
// Provides: {"impl_1581"}
// Dependencies: {}
impl LossThreshold { fn new (recovery_config : & RecoveryConfig) -> Self { let time_thresh_overhead = if recovery_config . enable_relaxed_loss_threshold { Some (INITIAL_TIME_THRESHOLD_OVERHEAD) } else { None } ; LossThreshold { pkt_thresh : Some (INITIAL_PACKET_THRESHOLD) , time_thresh : INITIAL_TIME_THRESHOLD , time_thresh_overhead , } } fn pkt_thresh (& self) -> Option < u64 > { self . pkt_thresh } fn time_thresh (& self) -> f64 { self . time_thresh } fn on_spurious_loss (& mut self , new_pkt_thresh : u64) { match & mut self . time_thresh_overhead { Some (time_thresh_overhead) => { if self . pkt_thresh . is_some () { self . pkt_thresh = None ; } else { * time_thresh_overhead *= TIME_THRESHOLD_OVERHEAD_MULTIPLIER ; * time_thresh_overhead = time_thresh_overhead . min (1.0) ; self . time_thresh = 1.0 + * time_thresh_overhead ; } } , None => { let new_packet_threshold = self . pkt_thresh . expect ("packet threshold should always be Some when `enable_relaxed_loss_threshold` is false") . max (new_pkt_thresh . min (MAX_PACKET_THRESHOLD)) ; self . pkt_thresh = Some (new_packet_threshold) ; self . time_thresh = PACKET_REORDER_TIME_THRESHOLD ; } , } } }
};
}
