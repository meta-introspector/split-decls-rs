// Generated macro for QUIC_UNSTAKED_RECEIVE_WINDOW_RATIO (const)
macro_rules! DepcrateQUIC_UNSTAKED_RECEIVE_WINDOW_RATIO {
() => {
// Module: crate
// Provides: {"QUIC_UNSTAKED_RECEIVE_WINDOW_RATIO"}
// Dependencies: {}
# [doc = " The receive window for QUIC connection from unstaked nodes is"] # [doc = " set to this ratio times [`solana_packet::PACKET_DATA_SIZE`]"] # [doc = ""] # [doc = " [`solana_packet::PACKET_DATA_SIZE`]: https://docs.rs/solana-packet/latest/solana_packet/constant.PACKET_DATA_SIZE.html"] pub const QUIC_UNSTAKED_RECEIVE_WINDOW_RATIO : u64 = 128 ;
};
}
