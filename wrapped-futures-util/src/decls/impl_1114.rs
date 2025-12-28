macro_rules! deps {
    () => {
        Cursor!();
    };
}

macro_rules! impl_1114 {
    () => {
        deps!();
        impl AsyncWrite for Cursor < & mut [u8] > { delegate_async_write_to_stdio ! () ; }
    };
}

impl_1114!();