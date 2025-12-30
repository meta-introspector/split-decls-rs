// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
# [allow (clippy :: uninit_assumed_init)] impl Default for Packet { fn default () -> Self { let buffer = std :: mem :: MaybeUninit :: < [u8 ; PACKET_DATA_SIZE] > :: uninit () ; Self { buffer : unsafe { buffer . assume_init () } , meta : Meta :: default () , } } }
};
}
