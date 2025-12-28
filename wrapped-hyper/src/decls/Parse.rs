macro_rules! deps {
    () => {
        Header!();
    };
}

macro_rules! Parse {
    () => {
        deps!();
        # [derive (Debug)] pub (super) enum Parse { Method , # [cfg (feature = "http1")] Version , # [cfg (all (any (feature = "client" , feature = "server") , feature = "http1"))] VersionH2 , Uri , # [cfg (all (feature = "http1" , feature = "server"))] UriTooLong , # [cfg (feature = "http1")] Header (Header) , # [cfg (any (feature = "http1" , feature = "http2"))] # [cfg_attr (feature = "http2" , allow (unused))] TooLarge , Status , # [cfg (all (any (feature = "client" , feature = "server") , feature = "http1"))] Internal , }
    };
}

Parse!();