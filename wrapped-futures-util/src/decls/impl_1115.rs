macro_rules! deps {
    () => {
        Cursor!();
    };
}

macro_rules! impl_1115 {
    () => {
        deps!();
        impl AsyncWrite for Cursor < & mut Vec < u8 > > { delegate_async_write_to_stdio ! () ; }
    };
}

impl_1115!();