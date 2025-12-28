macro_rules! deps {
    () => {
        SendError!();
        SendTimeoutError!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl < T > From < SendError < T > > for SendTimeoutError < T > { fn from (err : SendError < T >) -> Self { match err { SendError (e) => Self :: Disconnected (e) , } } }
    };
}

impl_82!()