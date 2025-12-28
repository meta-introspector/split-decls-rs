macro_rules! deps {
    () => {
        Read!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl < T : ? Sized + Read + Unpin > Read for & mut T { deref_async_read ! () ; }
    };
}

impl_173!();