macro_rules! deps {
    () => {
        AtomicOrdering!();
    };
}

macro_rules! impl_449 {
    () => {
        deps!();
        impl AtomicOrdering { pub (crate) fn from_generic (ao : rustc_middle :: ty :: AtomicOrdering) -> Self { use rustc_middle :: ty :: AtomicOrdering as Common ; match ao { Common :: Relaxed => Self :: Monotonic , Common :: Acquire => Self :: Acquire , Common :: Release => Self :: Release , Common :: AcqRel => Self :: AcquireRelease , Common :: SeqCst => Self :: SequentiallyConsistent , } } }
    };
}

impl_449!();