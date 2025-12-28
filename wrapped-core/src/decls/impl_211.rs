macro_rules! deps {
    () => {
        Weak!();
        Interface!();
    };
}

macro_rules! impl_211 {
    () => {
        deps!();
        impl < I : Interface > Weak < I > { # [doc = " Creates a new `Weak` object without any backing object."] pub const fn new () -> Self { Self (None , PhantomData) } # [doc = " Attempts to upgrade the weak reference to a strong reference."] pub fn upgrade (& self) -> Option < I > { self . 0 . as_ref () . and_then (| inner | unsafe { inner . Resolve () . ok () }) } pub (crate) fn downgrade (source : & imp :: IWeakReferenceSource) -> Self { let reference = unsafe { source . GetWeakReference () . ok () } ; Self (reference , PhantomData) } }
    };
}

impl_211!()