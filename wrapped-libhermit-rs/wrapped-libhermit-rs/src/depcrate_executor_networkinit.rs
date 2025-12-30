// Generated macro for init (function)
macro_rules! Depcrate_executor_networkinit {
() => {
// Module: crate::executor::network
// Provides: {"init"}
// Dependencies: {}
pub (crate) fn init () { info ! ("Try to initialize network!") ; LOCAL_ENDPOINT . store (start_endpoint () , Ordering :: Relaxed) ; let mut guard = NIC . lock () ; * guard = NetworkInterface :: create () ; if let NetworkState :: Initialized (nic) = & mut * guard { let time = now () ; nic . poll_common (time) ; let wakeup_time = nic . poll_delay (time) . map (| d | crate :: arch :: processor :: get_timer_ticks () + d . total_micros ()) ; crate :: core_scheduler () . add_network_timer (wakeup_time) ; spawn (network_run ()) ; # [cfg (feature = "dhcpv4")] spawn (dhcpv4_run ()) ; } }
};
}
