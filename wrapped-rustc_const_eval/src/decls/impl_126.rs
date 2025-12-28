macro_rules! deps {
    () => {
        MayLeak!();
        MemoryKind!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl interpret :: MayLeak for MemoryKind { # [inline (always)] fn may_leak (self) -> bool { match self { MemoryKind :: Heap { was_made_global } => was_made_global , } } }
    };
}

impl_126!()