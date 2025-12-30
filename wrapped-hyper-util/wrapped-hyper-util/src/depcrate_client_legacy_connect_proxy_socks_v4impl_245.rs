// Generated macro for impl_245 (impl)
macro_rules! Depcrate_client_legacy_connect_proxy_socks_v4impl_245 {
() => {
// Module: crate::client::legacy::connect::proxy::socks::v4
// Provides: {"impl_245"}
// Dependencies: {}
impl SocksConfig { pub fn new (proxy : Uri) -> Self { Self { proxy , local_dns : false , } } async fn execute < T , E > (self , mut conn : T , host : String , port : u16) -> Result < T , SocksError < E > > where T : Read + Write + Unpin , { let address = match host . parse :: < IpAddr > () { Ok (IpAddr :: V6 (_)) => return Err (SocksV4Error :: IpV6 . into ()) , Ok (IpAddr :: V4 (ip)) => Address :: Socket (SocketAddrV4 :: new (ip , port)) , Err (_) => { if self . local_dns { (host , port) . to_socket_addrs () ? . find_map (| s | { if let SocketAddr :: V4 (v4) = s { Some (Address :: Socket (v4)) } else { None } }) . ok_or (SocksError :: DnsFailure) ? } else { Address :: Domain (host , port) } } } ; let mut send_buf = BytesMut :: with_capacity (1024) ; let mut recv_buf = BytesMut :: with_capacity (1024) ; let req = Request (& address) ; let n = req . write_to_buf (& mut send_buf) ? ; crate :: rt :: write_all (& mut conn , & send_buf [.. n]) . await ? ; let res : Response = super :: read_message (& mut conn , & mut recv_buf) . await ? ; if res . 0 == Status :: Success { Ok (conn) } else { Err (SocksV4Error :: Command (res . 0) . into ()) } } }
};
}
