macro_rules! impl_1065 {
    () => {
        impl < R : AsyncWrite > AsyncWrite for BufReader < R > { delegate_async_write ! (inner) ; }
    };
}

impl_1065!()