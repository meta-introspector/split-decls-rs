macro_rules! deps {
    () => {
        BoundAsyncness!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        impl BoundAsyncness { pub fn as_str (self) -> & 'static str { match self { Self :: Normal => "" , Self :: Async (_) => "async" , } } }
    };
}

impl_176!()