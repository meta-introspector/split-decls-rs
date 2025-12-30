// Generated macro for impl_158 (impl)
macro_rules! Depcrate_client_legacy_connect_httpimpl_158 {
() => {
// Module: crate::client::legacy::connect::http
// Provides: {"impl_158"}
// Dependencies: {}
impl ConnectingTcpRemote { async fn connect (& mut self , config : & Config) -> Result < TcpStream , ConnectError > { let mut err = None ; for addr in & mut self . addrs { debug ! ("connecting to {}" , addr) ; match connect (& addr , config , self . connect_timeout) ? . await { Ok (tcp) => { debug ! ("connected to {}" , addr) ; return Ok (tcp) ; } Err (mut e) => { trace ! ("connect error for {}: {:?}" , addr , e) ; e . addr = Some (addr) ; if err . is_none () { err = Some (e) ; } } } } match err { Some (e) => Err (e) , None => Err (ConnectError :: new ("tcp connect error" , std :: io :: Error :: new (std :: io :: ErrorKind :: NotConnected , "Network unreachable") ,)) , } } }
};
}
