// Generated macro for impl_1084 (impl)
macro_rules! Depcrate_recovery_bytes_in_flightimpl_1084 {
() => {
// Module: crate::recovery::bytes_in_flight
// Provides: {"impl_1084"}
// Dependencies: {}
impl BytesInFlight { # [doc = " Add to bytes in flight.  Record the start time when"] # [doc = " bytes_in_flight was 0 at the beginning of the function."] pub (crate) fn add (& mut self , delta : usize , now : Instant) { if delta == 0 { return ; } self . bytes_in_flight += delta ; if self . bytes_in_flight_interval_start . is_some () { self . update_in_flight_duration (now) ; } else { self . bytes_in_flight_interval_start = Some (now) ; } } # [doc = " Substract from bytes in flight.  If bytes_in_flight drops to 0,"] # [doc = " end the current bytes_in_flight >0 interval."] pub (crate) fn saturating_subtract (& mut self , delta : usize , now : Instant) { self . bytes_in_flight = self . bytes_in_flight . saturating_sub (delta) ; self . update_in_flight_duration (now) ; } # [doc = " Current bytes in flight."] pub (crate) fn get (& self) -> usize { self . bytes_in_flight } # [doc = " Returns true if there are 0 bytes in flight."] pub (crate) fn is_zero (& self) -> bool { self . bytes_in_flight == 0 } # [doc = " Total time during which bytes_in_flight was > 0."] pub (crate) fn get_duration (& self) -> Duration { self . closed_interval_duration + self . open_interval_duration } fn update_in_flight_duration (& mut self , now : Instant) { if let Some (start) = self . bytes_in_flight_interval_start { if self . bytes_in_flight == 0 { self . open_interval_duration = Duration :: ZERO ; self . closed_interval_duration += now - start ; self . bytes_in_flight_interval_start = None ; } else { self . open_interval_duration = now - start ; } } } }
};
}
