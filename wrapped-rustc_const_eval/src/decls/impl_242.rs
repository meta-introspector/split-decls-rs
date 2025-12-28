macro_rules! deps {
    () => {
        MemoryKind!();
        MayLeak!();
        Machine!();
    };
}

macro_rules! impl_242 {
    () => {
        deps!();
        impl < T : MayLeak > MayLeak for MemoryKind < T > { # [inline] fn may_leak (self) -> bool { match self { MemoryKind :: Stack => false , MemoryKind :: CallerLocation => true , MemoryKind :: Machine (k) => k . may_leak () , } } }
    };
}

impl_242!();