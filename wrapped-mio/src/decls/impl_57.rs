macro_rules! deps {
    () => {
        Event!();
        Iter!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < 'a > Iterator for Iter < 'a > { type Item = & 'a Event ; fn next (& mut self) -> Option < Self :: Item > { let ret = self . inner . inner . get (self . pos) . map (Event :: from_sys_event_ref) ; self . pos += 1 ; ret } fn size_hint (& self) -> (usize , Option < usize >) { let size = self . inner . inner . len () ; (size , Some (size)) } fn count (self) -> usize { self . inner . inner . len () } }
    };
}

impl_57!();