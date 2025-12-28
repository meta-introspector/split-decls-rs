macro_rules! Protocol {
    () => {
        # [cfg (feature = "http2")] # [doc = " Extension type representing the `:protocol` pseudo-header in HTTP/2."] # [doc = ""] # [doc = " The `Protocol` extension allows access to the value of the `:protocol` pseudo-header"] # [doc = " used by the [Extended CONNECT Protocol](https://datatracker.ietf.org/doc/html/rfc8441#section-4)."] # [doc = " This extension is only sent on HTTP/2 CONNECT requests, most commonly with the value `websocket`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use hyper::ext::Protocol;"] # [doc = " use http::{Request, Method, Version};"] # [doc = ""] # [doc = " let mut req = Request::new(());"] # [doc = " *req.method_mut() = Method::CONNECT;"] # [doc = " *req.version_mut() = Version::HTTP_2;"] # [doc = " req.extensions_mut().insert(Protocol::from_static(\"websocket\"));"] # [doc = " // Now the request will include the `:protocol` pseudo-header with value \"websocket\""] # [doc = " ```"] # [derive (Clone , Eq , PartialEq)] pub struct Protocol { inner : h2 :: ext :: Protocol , }
    };
}

Protocol!();