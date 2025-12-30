// Generated macro for Wakened (struct)
macro_rules! Depcrate_future_extWakened {
() => {
// Module: crate::future_ext
// Provides: {"Wakened"}
// Dependencies: {}
# [doc = " A future that only polls the inner future if it has been woken (after the initial poll)."] pub struct Wakened < T > { future : Pin < Box < T > > , woken : Arc < AtomicBool > , }
};
}
