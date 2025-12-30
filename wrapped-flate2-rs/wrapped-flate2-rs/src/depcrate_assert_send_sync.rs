// Generated macro for _assert_send_sync (function)
macro_rules! Depcrate_assert_send_sync {
() => {
// Module: crate
// Provides: {"_assert_send_sync"}
// Dependencies: {}
fn _assert_send_sync () { fn _assert_send_sync < T : Send + Sync > () { } _assert_send_sync :: < read :: DeflateEncoder < & [u8] > > () ; _assert_send_sync :: < read :: DeflateDecoder < & [u8] > > () ; _assert_send_sync :: < read :: ZlibEncoder < & [u8] > > () ; _assert_send_sync :: < read :: ZlibDecoder < & [u8] > > () ; _assert_send_sync :: < read :: GzEncoder < & [u8] > > () ; _assert_send_sync :: < read :: GzDecoder < & [u8] > > () ; _assert_send_sync :: < read :: MultiGzDecoder < & [u8] > > () ; _assert_send_sync :: < write :: DeflateEncoder < Vec < u8 > > > () ; _assert_send_sync :: < write :: DeflateDecoder < Vec < u8 > > > () ; _assert_send_sync :: < write :: ZlibEncoder < Vec < u8 > > > () ; _assert_send_sync :: < write :: ZlibDecoder < Vec < u8 > > > () ; _assert_send_sync :: < write :: GzEncoder < Vec < u8 > > > () ; _assert_send_sync :: < write :: GzDecoder < Vec < u8 > > > () ; }
};
}
