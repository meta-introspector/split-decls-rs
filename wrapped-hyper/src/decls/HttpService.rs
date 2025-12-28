macro_rules! deps {
    () => {
        Error!();
        Response!();
        Service!();
        Result!();
    };
}

macro_rules! HttpService {
    () => {
        deps!();
        # [doc = " An asynchronous function from [`Request`] to [`Response`]."] # [doc = ""] # [doc = " This is a *sealed* trait, meaning that it can not be implemented directly. Rather, it is an"] # [doc = " alias for [`Service`]s that accept a [`Request`] and return a [`Future`] that resolves to a"] # [doc = " [`Response`]. External callers should implement [`Service`] instead."] # [doc = ""] # [doc = " Rather than being generic over the request and response, this trait is generic across the"] # [doc = " request [`Body`] and response [`Body`]."] # [doc = ""] # [doc = " See the crate-level [`service`][crate::service] documentation for more information."] # [doc = ""] # [doc = " See [`Service`] for more information."] pub trait HttpService < ReqBody > : sealed :: Sealed < ReqBody > { # [doc = " The [`Body`] body of the [`Response`]."] type ResBody : Body ; # [doc = " The error type that can occur within this [`Service`]."] # [doc = ""] # [doc = " Note: Returning an `Error` to a hyper server, the behavior depends on the protocol. In"] # [doc = " most cases, hyper will cause the connection to be abruptly aborted. In most cases, it is"] # [doc = " better to return a `Response` with a 4xx or 5xx status code."] # [doc = ""] # [doc = " See [`Service::Error`] for more information."] type Error : Into < Box < dyn StdError + Send + Sync > > ; # [doc = " The [`Future`] returned by this [`Service`]."] type Future : Future < Output = Result < Response < Self :: ResBody > , Self :: Error > > ; # [doc (hidden)] fn call (& mut self , req : Request < ReqBody >) -> Self :: Future ; }
    };
}

HttpService!()