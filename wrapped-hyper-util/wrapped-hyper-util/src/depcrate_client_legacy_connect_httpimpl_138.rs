// Generated macro for impl_138 (impl)
macro_rules! Depcrate_client_legacy_connect_httpimpl_138 {
() => {
// Module: crate::client::legacy::connect::http
// Provides: {"impl_138"}
// Dependencies: {}
impl < R > HttpConnector < R > where R : Resolve , { async fn call_async (& mut self , dst : Uri) -> Result < TokioIo < TcpStream > , ConnectError > { let config = & self . config ; let (host , port) = get_host_port (config , & dst) ? ; let host = host . trim_start_matches ('[') . trim_end_matches (']') ; let addrs = if let Some (addrs) = dns :: SocketAddrs :: try_parse (host , port) { addrs } else { let addrs = resolve (& mut self . resolver , dns :: Name :: new (host . into ())) . await . map_err (ConnectError :: dns) ? ; let addrs = addrs . map (| mut addr | { set_port (& mut addr , port , dst . port () . is_some ()) ; addr }) . collect () ; dns :: SocketAddrs :: new (addrs) } ; let c = ConnectingTcp :: new (addrs , config) ; let sock = c . connect () . await ? ; if let Err (e) = sock . set_nodelay (config . nodelay) { warn ! ("tcp set_nodelay error: {}" , e) ; } Ok (TokioIo :: new (sock)) } }
};
}
