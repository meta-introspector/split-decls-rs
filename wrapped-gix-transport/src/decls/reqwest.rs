macro_rules! reqwest {
    () => {
        # [doc = " The experimental `reqwest` backend."] # [doc = ""] # [doc = " It doesn't support any of the shared http options yet, but can be seen as example on how to integrate blocking `http` backends."] # [doc = " There is also nothing that would prevent it from becoming a fully-featured HTTP backend except for demand and time."] # [cfg (feature = "http-client-reqwest")] pub mod reqwest ;
    };
}

reqwest!()