// Generated macro for tests (module)
macro_rules! Depcrate_sockettests {
() => {
// Module: crate::socket
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn socket_addr () { let addr = SocketAddr :: Tcp ("127.0.0.1:8080" . parse () . unwrap ()) ; assert ! (format ! ("{addr:?}") . contains ("127.0.0.1:8080")) ; assert_eq ! (format ! ("{addr}") , "127.0.0.1:8080") ; let addr : StdSocketAddr = "127.0.0.1:0" . parse () . unwrap () ; let lst = create_mio_tcp_listener (addr , 128 , & MpTcp :: Disabled) . unwrap () ; let lst = MioListener :: Tcp (lst) ; assert ! (format ! ("{lst:?}") . contains ("TcpListener")) ; assert ! (format ! ("{lst}") . contains ("127.0.0.1")) ; } # [test] # [cfg (unix)] fn uds () { let _ = std :: fs :: remove_file ("/tmp/sock.xxxxx") ; if let Ok (socket) = MioUnixListener :: bind ("/tmp/sock.xxxxx") { let addr = socket . local_addr () . expect ("Couldn't get local address") ; let a = SocketAddr :: Uds (addr) ; assert ! (format ! ("{a:?}") . contains ("/tmp/sock.xxxxx")) ; assert ! (format ! ("{a}") . contains ("/tmp/sock.xxxxx")) ; let lst = MioListener :: Uds (socket) ; assert ! (format ! ("{lst:?}") . contains ("/tmp/sock.xxxxx")) ; assert ! (format ! ("{lst}") . contains ("/tmp/sock.xxxxx")) ; } } }
};
}
