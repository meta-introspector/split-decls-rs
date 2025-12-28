macro_rules! deps {
    () => {
        Error!();
        Protocol!();
        Connection!();
        ConnectMode!();
    };
}

macro_rules! connect {
    () => {
        deps!();
        # [doc = ""] pub mod connect { use std :: net :: { TcpStream , ToSocketAddrs } ; use bstr :: BString ; use super :: Connection ; use crate :: client :: git ; # [doc = " The error used in [`connect()`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("An IO error occurred when connecting to the server")] Io (# [from] std :: io :: Error) , # [error ("Could not parse {host:?} as virtual host with format <host>[:port]")] VirtualHostInvalid { host : String } , } impl crate :: IsSpuriousError for Error { fn is_spurious (& self) -> bool { match self { Error :: Io (err) => err . is_spurious () , _ => false , } } } fn parse_host (input : String) -> Result < (String , Option < u16 >) , Error > { let mut tokens = input . splitn (2 , ':') ; Ok (match (tokens . next () , tokens . next ()) { (Some (host) , None) => (host . to_owned () , None) , (Some (host) , Some (port)) => (host . to_owned () , Some (port . parse () . map_err (| _ | Error :: VirtualHostInvalid { host : input }) ?) ,) , _ => unreachable ! ("we expect at least one token, the original string") , }) } # [doc = " Connect to a git daemon running on `host` and optionally `port` and a repository at `path`."] # [doc = ""] # [doc = " Use `desired_version` to specify a preferred protocol to use, knowing that it can be downgraded by a server not supporting it."] # [doc = " If `trace` is `true`, all packetlines received or sent will be passed to the facilities of the `gix-trace` crate."] pub fn connect (host : & str , path : BString , desired_version : crate :: Protocol , port : Option < u16 > , trace : bool ,) -> Result < Connection < TcpStream , TcpStream > , Error > { let read = TcpStream :: connect_timeout (& (host , port . unwrap_or (9418)) . to_socket_addrs () ? . next () . expect ("after successful resolution there is an IP address") , std :: time :: Duration :: from_secs (5) ,) ? ; let write = read . try_clone () ? ; let vhost = std :: env :: var ("GIT_OVERRIDE_VIRTUAL_HOST") . ok () . map (parse_host) . transpose () ? . unwrap_or_else (| | (host . to_owned () , port)) ; Ok (Connection :: new (read , write , desired_version , path , Some (vhost) , git :: ConnectMode :: Daemon , trace ,)) } }
    };
}

connect!();