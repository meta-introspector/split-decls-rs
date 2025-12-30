// Generated macro for impl_426 (impl)
macro_rules! Depcrate_connection_pathsimpl_426 {
() => {
// Module: crate::connection::paths
// Provides: {"impl_426"}
// Dependencies: {}
impl RttEstimator { fn new (initial_rtt : Duration) -> Self { Self { latest : initial_rtt , smoothed : None , var : initial_rtt / 2 , min : initial_rtt , } } # [doc = " The current best RTT estimation."] pub fn get (& self) -> Duration { self . smoothed . unwrap_or (self . latest) } # [doc = " Conservative estimate of RTT"] # [doc = ""] # [doc = " Takes the maximum of smoothed and latest RTT, as recommended"] # [doc = " in 6.1.2 of the recovery spec (draft 29)."] pub fn conservative (& self) -> Duration { self . get () . max (self . latest) } # [doc = " Minimum RTT registered so far for this estimator."] pub fn min (& self) -> Duration { self . min } pub (crate) fn pto_base (& self) -> Duration { self . get () + cmp :: max (4 * self . var , TIMER_GRANULARITY) } pub (crate) fn update (& mut self , ack_delay : Duration , rtt : Duration) { self . latest = rtt ; self . min = cmp :: min (self . min , self . latest) ; if let Some (smoothed) = self . smoothed { let adjusted_rtt = if self . min + ack_delay <= self . latest { self . latest - ack_delay } else { self . latest } ; let var_sample = if smoothed > adjusted_rtt { smoothed - adjusted_rtt } else { adjusted_rtt - smoothed } ; self . var = (3 * self . var + var_sample) / 4 ; self . smoothed = Some ((7 * smoothed + adjusted_rtt) / 8) ; } else { self . smoothed = Some (self . latest) ; self . var = self . latest / 2 ; self . min = self . latest ; } } }
};
}
