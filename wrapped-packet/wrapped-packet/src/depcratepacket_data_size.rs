// Generated macro for PACKET_DATA_SIZE (const)
macro_rules! DepcratePACKET_DATA_SIZE {
() => {
// Module: crate
// Provides: {"PACKET_DATA_SIZE"}
// Dependencies: {}
# [doc = " Maximum over-the-wire size of a Transaction"] # [doc = "   1280 is IPv6 minimum MTU"] # [doc = "   40 bytes is the size of the IPv6 header"] # [doc = "   8 bytes is the size of the fragment header"] pub const PACKET_DATA_SIZE : usize = 1280 - 40 - 8 ;
};
}
