macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < I > Iterator for Iter < '_ , I > where I : Iterator , { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { if self . should_interrupt . load (Ordering :: Relaxed) { return None ; } self . inner . next () } }
    };
}

impl_17!()