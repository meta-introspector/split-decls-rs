macro_rules! deps {
    () => {
        Write!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        impl < T : ? Sized + Write + Unpin > Write for Box < T > { deref_async_write ! () ; }
    };
}

impl_176!()