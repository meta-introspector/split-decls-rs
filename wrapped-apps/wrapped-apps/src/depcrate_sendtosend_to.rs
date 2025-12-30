// Generated macro for send_to (function)
macro_rules! Depcrate_sendtosend_to {
() => {
// Module: crate::sendto
// Provides: {"send_to"}
// Dependencies: {}
# [doc = " A wrapper function of send_to()."] # [doc = ""] # [doc = " When GSO and SO_TXTIME are enabled, send packets using send_to_gso()."] # [doc = " Otherwise, send packets using socket.send_to()."] pub fn send_to (socket : & mio :: net :: UdpSocket , buf : & [u8] , send_info : & quiche :: SendInfo , segment_size : usize , pacing : bool , enable_gso : bool ,) -> io :: Result < usize > { if pacing && enable_gso { match send_to_gso_pacing (socket , buf , send_info , segment_size) { Ok (v) => { return Ok (v) ; } , Err (e) => { return Err (e) ; } , } } let mut off = 0 ; let mut left = buf . len () ; let mut written = 0 ; while left > 0 { let pkt_len = cmp :: min (left , segment_size) ; match socket . send_to (& buf [off .. off + pkt_len] , send_info . to) { Ok (v) => { written += v ; } , Err (e) => return Err (e) , } off += pkt_len ; left -= pkt_len ; } Ok (written) }
};
}
