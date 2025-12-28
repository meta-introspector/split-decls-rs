macro_rules! deps {
    () => {
        Io!();
        Parse!();
        Error!();
        User!();
    };
}

macro_rules! Kind {
    () => {
        deps!();
        # [derive (Debug)] pub (super) enum Kind { Parse (Parse) , User (User) , # [doc = " A message reached EOF, but is not complete."] # [cfg (all (any (feature = "client" , feature = "server") , feature = "http1"))] IncompleteMessage , # [doc = " A connection received a message (or bytes) when not waiting for one."] # [cfg (all (any (feature = "client" , feature = "server") , feature = "http1"))] UnexpectedMessage , # [doc = " A pending item was dropped before ever being processed."] Canceled , # [doc = " Indicates a channel (client or body sender) is closed."] # [cfg (any (all (feature = "http1" , any (feature = "client" , feature = "server")) , all (feature = "http2" , feature = "client")))] ChannelClosed , # [doc = " An `io::Error` that occurred while trying to read or write to a network stream."] # [cfg (all (any (feature = "client" , feature = "server") , any (feature = "http1" , feature = "http2")))] Io , # [doc = " User took too long to send headers"] # [cfg (all (feature = "http1" , feature = "server"))] HeaderTimeout , # [doc = " Error while reading a body from connection."] # [cfg (all (any (feature = "client" , feature = "server") , any (feature = "http1" , feature = "http2")))] Body , # [doc = " Error while writing a body to connection."] # [cfg (all (any (feature = "client" , feature = "server") , any (feature = "http1" , feature = "http2")))] BodyWrite , # [doc = " Error calling AsyncWrite::shutdown()"] # [cfg (all (any (feature = "client" , feature = "server") , feature = "http1"))] Shutdown , # [doc = " A general error from h2."] # [cfg (all (any (feature = "client" , feature = "server") , feature = "http2"))] Http2 , }
    };
}

Kind!()