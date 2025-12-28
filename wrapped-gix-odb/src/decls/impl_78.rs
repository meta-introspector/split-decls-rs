macro_rules! deps {
    () => {
        IncOnNewAndDecOnDrop!();
        Ordering!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < 'a > IncOnNewAndDecOnDrop < 'a > { pub fn new (v : & 'a AtomicU16) -> Self { v . fetch_add (1 , Ordering :: SeqCst) ; Self (v) } }
    };
}

impl_78!();