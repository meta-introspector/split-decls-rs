// Generated macro for connect_with_buffer_factory (function)
macro_rules! Depcrateconnect_with_buffer_factory {
() => {
// Module: crate
// Provides: {"connect_with_buffer_factory"}
// Dependencies: {}
# [doc = " Creates a new client-side connection, with a custom buffer generation"] # [doc = " method."] # [doc = ""] # [doc = " The buffers generated can be anything that can be drereferenced as a byte"] # [doc = " slice. See [`connect`] and [`BufFactory`] for more info."] # [inline] pub fn connect_with_buffer_factory < F : BufFactory > (server_name : Option < & str > , scid : & ConnectionId , local : SocketAddr , peer : SocketAddr , config : & mut Config ,) -> Result < Connection < F > > { let mut conn = Connection :: new (scid , None , local , peer , config , false) ? ; if let Some (server_name) = server_name { conn . handshake . set_host_name (server_name) ? ; } Ok (conn) }
};
}
