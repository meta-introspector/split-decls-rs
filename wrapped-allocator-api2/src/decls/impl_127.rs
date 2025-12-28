macro_rules! deps {
    () => {
        IntoIter!();
        Allocator!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl < T , A : Allocator > DoubleEndedIterator for IntoIter < T , A > { # [inline (always)] fn next_back (& mut self) -> Option < T > { if self . end == self . ptr { None } else if size_of :: < T > () == 0 { self . end = self . end . cast :: < u8 > () . wrapping_add (1) . cast () ; Some (unsafe { mem :: zeroed () }) } else { self . end = unsafe { self . end . sub (1) } ; Some (unsafe { ptr :: read (self . end) }) } } }
    };
}

impl_127!()