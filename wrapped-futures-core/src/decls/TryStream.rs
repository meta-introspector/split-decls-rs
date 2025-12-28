macro_rules! deps {
    () => {
        Stream!();
    };
}

macro_rules! TryStream {
    () => {
        deps!();
        # [doc = " A convenience for streams that return `Result` values that includes"] # [doc = " a variety of adapters tailored to such futures."] pub trait TryStream : Stream + private_try_stream :: Sealed { # [doc = " The type of successful values yielded by this future"] type Ok ; # [doc = " The type of failures yielded by this future"] type Error ; # [doc = " Poll this `TryStream` as if it were a `Stream`."] # [doc = ""] # [doc = " This method is a stopgap for a compiler limitation that prevents us from"] # [doc = " directly inheriting from the `Stream` trait; in the future it won't be"] # [doc = " needed."] fn try_poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < Self :: Ok , Self :: Error > > > ; }
    };
}

TryStream!()