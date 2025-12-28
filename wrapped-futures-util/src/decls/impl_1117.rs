macro_rules! deps {
    () => {
        Cursor!();
    };
}

macro_rules! impl_1117 {
    () => {
        deps!();
        impl AsyncWrite for Cursor < Box < [u8] > > { delegate_async_write_to_stdio ! () ; }
    };
}

impl_1117!();