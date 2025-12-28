macro_rules! deps {
    () => {
        ConnectionState!();
    };
}

macro_rules! Connection {
    () => {
        deps!();
        # [doc = " A TCP connection to either a `git` daemon or a spawned `git` process."] # [doc = ""] # [doc = " When connecting to a daemon, additional context information is sent with the first line of the handshake. Otherwise that"] # [doc = " context is passed using command line arguments to a [spawned `git` process][crate::client::blocking_io::file::SpawnProcessOnDemand]."] pub struct Connection < R , W > { pub (in crate :: client) writer : W , pub (in crate :: client) line_provider : StreamingPeekableIter < R > , pub (in crate :: client) state : ConnectionState , }
    };
}

Connection!();