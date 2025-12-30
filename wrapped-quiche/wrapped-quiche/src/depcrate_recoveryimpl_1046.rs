// Generated macro for impl_1046 (impl)
macro_rules! Depcrate_recoveryimpl_1046 {
() => {
// Module: crate::recovery
// Provides: {"impl_1046"}
// Dependencies: {}
impl Recovery { pub fn new_with_config (recovery_config : & RecoveryConfig) -> Self { let grecovery = GRecovery :: new (recovery_config) ; if let Some (grecovery) = grecovery { Recovery :: from (grecovery) } else { Recovery :: from (LegacyRecovery :: new_with_config (recovery_config)) } } # [cfg (feature = "qlog")] pub fn maybe_qlog (& mut self , qlog : & mut qlog :: streamer :: QlogStreamer , now : Instant ,) { if let Some (ev_data) = self . get_updated_qlog_event_data () { qlog . add_event_data_with_instant (ev_data , now) . ok () ; } if let Some (cc_state) = self . get_updated_qlog_cc_state (now) { let ev_data = EventData :: CongestionStateUpdated (qlog :: events :: quic :: CongestionStateUpdated { old : None , new : cc_state . to_string () , trigger : None , } ,) ; qlog . add_event_data_with_instant (ev_data , now) . ok () ; } } # [cfg (test)] pub fn new (config : & Config) -> Self { Self :: new_with_config (& RecoveryConfig :: from_config (config)) } }
};
}
