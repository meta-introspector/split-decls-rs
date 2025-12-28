macro_rules! impl_1075 {
    () => {
        impl < W : AsyncRead > AsyncRead for BufWriter < W > { delegate_async_read ! (inner) ; }
    };
}

impl_1075!()