macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_465 {
    () => {
        deps!();
        impl < 'a , T > Iterator for Iter < 'a , T > { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { if self . index < self . len { let head = self . rb . head . load (Ordering :: Relaxed) ; let i = (head + self . index) % self . rb . n () ; self . index += 1 ; Some (unsafe { & * (self . rb . buffer . borrow () . get_unchecked (i) . get () as * const T) }) } else { None } } }
    };
}

impl_465!();