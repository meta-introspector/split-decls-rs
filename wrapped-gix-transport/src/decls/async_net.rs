macro_rules! deps {
    () => {
        Connection!();
        Error!();
        Protocol!();
        ConnectMode!();
    };
}

macro_rules! async_net {
    () => {
        deps!();
        # [cfg (feature = "async-std")] mod async_net { use std :: time :: Duration ; use async_std :: net :: TcpStream ; use crate :: client :: { git :: { async_io :: Connection , ConnectMode } , Error , } ; impl Connection < TcpStream , TcpStream > { # [doc = " Create a new TCP connection using the `git` protocol of `desired_version`, and make a connection to `host`"] # [doc = " at `port` for accessing the repository at `path` on the server side."] # [doc = " If `trace` is `true`, all packetlines received or sent will be passed to the facilities of the `gix-trace` crate."] pub async fn new_tcp (host : & str , port : Option < u16 > , path : bstr :: BString , desired_version : crate :: Protocol , trace : bool ,) -> Result < Self , Error > { let read = async_std :: io :: timeout (Duration :: from_secs (5) , TcpStream :: connect (& (host , port . unwrap_or (9418))) ,) . await ? ; let write = read . clone () ; Ok (Self :: new (read , write , desired_version , path , None :: < (String , _) > , ConnectMode :: Daemon , trace ,)) } } }
    };
}

async_net!()