// Generated macro for AckFrequencyConfig (struct)
macro_rules! Depcrate_config_transportAckFrequencyConfig {
() => {
// Module: crate::config::transport
// Provides: {"AckFrequencyConfig"}
// Dependencies: {}
# [doc = " Parameters for controlling the peer's acknowledgement frequency"] # [doc = ""] # [doc = " The parameters provided in this config will be sent to the peer at the beginning of the"] # [doc = " connection, so it can take them into account when sending acknowledgements (see each parameter's"] # [doc = " description for details on how it influences acknowledgement frequency)."] # [doc = ""] # [doc = " Quinn's implementation follows the fourth draft of the"] # [doc = " [QUIC Acknowledgement Frequency extension](https://datatracker.ietf.org/doc/html/draft-ietf-quic-ack-frequency-04)."] # [doc = " The defaults produce behavior slightly different than the behavior without this extension,"] # [doc = " because they change the way reordered packets are handled (see"] # [doc = " [`AckFrequencyConfig::reordering_threshold`] for details)."] # [derive (Clone , Debug)] pub struct AckFrequencyConfig { pub (crate) ack_eliciting_threshold : VarInt , pub (crate) max_ack_delay : Option < Duration > , pub (crate) reordering_threshold : VarInt , }
};
}
