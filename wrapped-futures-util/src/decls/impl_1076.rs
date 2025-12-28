macro_rules! impl_1076 {
    () => {
        impl < W : AsyncBufRead > AsyncBufRead for BufWriter < W > { delegate_async_buf_read ! (inner) ; }
    };
}

impl_1076!()