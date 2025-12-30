// Generated macro for accept_with_buf_factory (function)
macro_rules! Depcrateaccept_with_buf_factory {
() => {
// Module: crate
// Provides: {"accept_with_buf_factory"}
// Dependencies: {}
# [doc = " Creates a new server-side connection, with a custom buffer generation"] # [doc = " method."] # [doc = ""] # [doc = " The buffers generated can be anything that can be drereferenced as a byte"] # [doc = " slice. See [`accept`] and [`BufFactory`] for more info."] # [inline] pub fn accept_with_buf_factory < F : BufFactory > (scid : & ConnectionId , odcid : Option < & ConnectionId > , local : SocketAddr , peer : SocketAddr , config : & mut Config ,) -> Result < Connection < F > > { let conn = Connection :: new (scid , odcid , local , peer , config , true) ? ; Ok (conn) }
};
}
