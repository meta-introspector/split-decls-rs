macro_rules! deps {
    () => {
        Allocator!();
        Box!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < I : DoubleEndedIterator + ? Sized , A : Allocator > DoubleEndedIterator for Box < I , A > { # [inline (always)] fn next_back (& mut self) -> Option < I :: Item > { (* * self) . next_back () } # [inline (always)] fn nth_back (& mut self , n : usize) -> Option < I :: Item > { (* * self) . nth_back (n) } }
    };
}

impl_57!()