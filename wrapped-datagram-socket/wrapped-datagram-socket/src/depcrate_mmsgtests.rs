// Generated macro for tests (module)
macro_rules! Depcrate_mmsgtests {
() => {
// Module: crate::mmsg
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: io ; use tokio :: io :: ReadBuf ; use tokio :: net :: UnixDatagram ; use crate :: DatagramSocketRecvExt ; use crate :: DatagramSocketSendExt ; # [tokio :: test] async fn recvmmsg () -> io :: Result < () > { let (s , mut r) = UnixDatagram :: pair () ? ; let mut bufs = [[0u8 ; 128] ; 128] ; for i in 0 .. 5 { s . send (& [i ; 128]) . await ? ; } let mut rbufs : Vec < _ > = bufs . iter_mut () . map (| s | ReadBuf :: new (& mut s [..])) . collect () ; assert_eq ! (r . recv_many (& mut rbufs) . await ?, 5) ; for (i , buf) in rbufs [0 .. 5] . iter () . enumerate () { assert_eq ! (buf . filled () , & [i as u8 ; 128]) ; } for i in 0 .. 92 { s . send (& [i ; 128]) . await ? ; } let mut rbufs : Vec < _ > = bufs . iter_mut () . map (| s | ReadBuf :: new (& mut s [..])) . collect () ; assert_eq ! (r . recv_many (& mut rbufs) . await ?, 92) ; for (i , buf) in rbufs [0 .. 92] . iter () . enumerate () { assert_eq ! (buf . filled () , & [i as u8 ; 128]) ; } Ok (()) } # [tokio :: test] async fn sendmmsg () -> io :: Result < () > { let (s , r) = UnixDatagram :: pair () ? ; let mut bufs : [_ ; 128] = std :: array :: from_fn (| i | [i as u8 ; 128]) ; let wbufs : Vec < _ > = bufs . iter_mut () . map (| s | { let mut b = ReadBuf :: new (& mut s [..]) ; b . set_filled (128) ; b }) . collect () ; assert_eq ! (s . send_many (& wbufs [.. 5]) . await ?, 5) ; let mut rbuf = [0u8 ; 128] ; for i in 0 .. 5 { assert_eq ! (r . recv (& mut rbuf) . await ?, 128) ; assert_eq ! (rbuf , [i as u8 ; 128]) ; } Ok (()) } }
};
}
