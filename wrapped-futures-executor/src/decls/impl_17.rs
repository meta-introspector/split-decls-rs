macro_rules! deps {
    () => {
        BlockingStream!();
        LocalPool!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < S : Stream + Unpin > Iterator for BlockingStream < S > { type Item = S :: Item ; fn next (& mut self) -> Option < Self :: Item > { LocalPool :: new () . run_until (self . stream . next ()) } fn size_hint (& self) -> (usize , Option < usize >) { self . stream . size_hint () } }
    };
}

impl_17!();