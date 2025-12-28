macro_rules! deps {
    () => {
        Allocator!();
        Splice!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < I : Iterator , A : Allocator > Iterator for Splice < '_ , I , A > { type Item = I :: Item ; # [inline (always)] fn next (& mut self) -> Option < Self :: Item > { self . drain . next () } # [inline (always)] fn size_hint (& self) -> (usize , Option < usize >) { self . drain . size_hint () } }
    };
}

impl_102!()