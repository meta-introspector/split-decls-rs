// Generated macro for SpaceId (enum)
macro_rules! Depcrate_packetSpaceId {
() => {
// Module: crate::packet
// Provides: {"SpaceId"}
// Dependencies: {}
# [doc = " Packet number space identifiers"] # [derive (Debug , Copy , Clone , Eq , PartialEq , Ord , PartialOrd)] pub enum SpaceId { # [doc = " Unprotected packets, used to bootstrap the handshake"] Initial = 0 , Handshake = 1 , # [doc = " Application data space, used for 0-RTT and post-handshake/1-RTT packets"] Data = 2 , }
};
}
