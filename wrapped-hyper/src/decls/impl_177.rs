macro_rules! deps {
    () => {
        Write!();
    };
}

macro_rules! impl_177 {
    () => {
        deps!();
        impl < T : ? Sized + Write + Unpin > Write for & mut T { deref_async_write ! () ; }
    };
}

impl_177!()