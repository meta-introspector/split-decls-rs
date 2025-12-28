macro_rules! deps {
    () => {
        CollectResult!();
        SendPtr!();
    };
}

macro_rules! CollectConsumer {
    () => {
        deps!();
        pub (super) struct CollectConsumer < 'c , T : Send > { # [doc = " See `CollectResult` for explanation of why this is not a slice"] start : SendPtr < T > , len : usize , marker : PhantomData < & 'c mut T > , }
    };
}

CollectConsumer!()