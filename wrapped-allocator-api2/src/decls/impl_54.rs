macro_rules! deps {
    () => {
        BoxIter!();
        Box!();
        Allocator!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < I : Iterator + ? Sized , A : Allocator > Iterator for Box < I , A > { type Item = I :: Item ; # [inline (always)] fn next (& mut self) -> Option < I :: Item > { (* * self) . next () } # [inline (always)] fn size_hint (& self) -> (usize , Option < usize >) { (* * self) . size_hint () } # [inline (always)] fn nth (& mut self , n : usize) -> Option < I :: Item > { (* * self) . nth (n) } # [inline (always)] fn last (self) -> Option < I :: Item > { BoxIter :: last (self) } }
    };
}

impl_54!();