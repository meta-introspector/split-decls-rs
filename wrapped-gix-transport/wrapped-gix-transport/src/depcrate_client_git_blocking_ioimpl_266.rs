// Generated macro for impl_266 (impl)
macro_rules! Depcrate_client_git_blocking_ioimpl_266 {
() => {
// Module: crate::client::git::blocking_io
// Provides: {"impl_266"}
// Dependencies: {}
impl < R , W > Connection < R , W > where R : std :: io :: Read , W : std :: io :: Write , { # [doc = " Create a connection from the given `read` and `write`, asking for `desired_version` as preferred protocol"] # [doc = " and the transfer of the repository at `repository_path`."] # [doc = ""] # [doc = " `virtual_host` along with a port to which to connect to, while `mode` determines the kind of endpoint to connect to."] # [doc = " If `trace` is `true`, all packetlines received or sent will be passed to the facilities of the `gix-trace` crate."] pub fn new (read : R , write : W , desired_version : Protocol , repository_path : impl Into < BString > , virtual_host : Option < (impl Into < String > , Option < u16 >) > , mode : git :: ConnectMode , trace : bool ,) -> Self { Connection { writer : write , line_provider : StreamingPeekableIter :: new (read , & [PacketLineRef :: Flush] , trace) , state : ConnectionState { path : repository_path . into () , virtual_host : virtual_host . map (| (h , p) | (h . into () , p)) , desired_version , custom_url : None , mode , } , } } pub (crate) fn new_for_spawned_process (reader : R , writer : W , desired_version : Protocol , repository_path : impl Into < BString > , trace : bool ,) -> Self { Self :: new (reader , writer , desired_version , repository_path , None :: < (& str , _) > , git :: ConnectMode :: Process , trace ,) } }
};
}
