macro_rules! deps {
    () => {
        FARPROC!();
        Symbol!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < T > core :: ops :: Deref for Symbol < T > { type Target = T ; fn deref (& self) -> & T { unsafe { & * ((& self . pointer) as * const FARPROC as * const T) } } }
    };
}

impl_120!()