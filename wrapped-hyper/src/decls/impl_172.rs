macro_rules! deps {
    () => {
        Read!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        impl < T : ? Sized + Read + Unpin > Read for Box < T > { deref_async_read ! () ; }
    };
}

impl_172!();