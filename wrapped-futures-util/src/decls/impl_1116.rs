macro_rules! deps {
    () => {
        Cursor!();
    };
}

macro_rules! impl_1116 {
    () => {
        deps!();
        impl AsyncWrite for Cursor < Vec < u8 > > { delegate_async_write_to_stdio ! () ; }
    };
}

impl_1116!()