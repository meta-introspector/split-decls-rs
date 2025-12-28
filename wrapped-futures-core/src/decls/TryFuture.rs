macro_rules! TryFuture {
    () => {
        # [doc = " A convenience for futures that return `Result` values that includes"] # [doc = " a variety of adapters tailored to such futures."] pub trait TryFuture : Future + private_try_future :: Sealed { # [doc = " The type of successful values yielded by this future"] type Ok ; # [doc = " The type of failures yielded by this future"] type Error ; # [doc = " Poll this `TryFuture` as if it were a `Future`."] # [doc = ""] # [doc = " This method is a stopgap for a compiler limitation that prevents us from"] # [doc = " directly inheriting from the `Future` trait; in the future it won't be"] # [doc = " needed."] fn try_poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < Self :: Ok , Self :: Error > > ; }
    };
}

TryFuture!()