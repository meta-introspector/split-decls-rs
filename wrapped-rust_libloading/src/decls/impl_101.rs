macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl < T > core :: ops :: Deref for Symbol < T > { type Target = T ; fn deref (& self) -> & T { unsafe { & * (& self . pointer as * const * mut _ as * const T) } } }
    };
}

impl_101!()