macro_rules! deps {
    () => {
        Protocol!();
        ConnectionState!();
        Connection!();
        ConnectMode!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl < R , W > Connection < R , W > where R : AsyncRead + Unpin , W : AsyncWrite + Unpin , { # [doc = " Create a connection from the given `read` and `write`, asking for `desired_version` as preferred protocol"] # [doc = " and the transfer of the repository at `repository_path`."] # [doc = ""] # [doc = " `virtual_host` along with a port to which to connect to, while `mode` determines the kind of endpoint to connect to."] # [doc = " If `trace` is `true`, all packetlines received or sent will be passed to the facilities of the `gix-trace` crate."] pub fn new (read : R , write : W , desired_version : Protocol , repository_path : impl Into < BString > , virtual_host : Option < (impl Into < String > , Option < u16 >) > , mode : git :: ConnectMode , trace : bool ,) -> Self { Connection { writer : write , line_provider : StreamingPeekableIter :: new (read , & [PacketLineRef :: Flush] , trace) , state : ConnectionState { path : repository_path . into () , virtual_host : virtual_host . map (| (h , p) | (h . into () , p)) , desired_version , custom_url : None , mode , } , } } }
    };
}

impl_170!();