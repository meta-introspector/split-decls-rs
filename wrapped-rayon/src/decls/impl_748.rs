macro_rules! deps {
    () => {
        PanicFuseIter!();
    };
}

macro_rules! impl_748 {
    () => {
        deps!();
        impl < 'a , I > ExactSizeIterator for PanicFuseIter < 'a , I > where I : ExactSizeIterator , { fn len (& self) -> usize { self . base . len () } }
    };
}

impl_748!();