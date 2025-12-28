macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_468 {
    () => {
        deps!();
        impl < T > DoubleEndedIterator for IterMut < '_ , T > { fn next_back (& mut self) -> Option < Self :: Item > { if self . index < self . len { let head = self . rb . head . load (Ordering :: Relaxed) ; let i = (head + self . len - 1) % self . rb . n () ; self . len -= 1 ; Some (unsafe { & mut * self . rb . buffer . borrow () . get_unchecked (i) . get () . cast :: < T > () }) } else { None } } }
    };
}

impl_468!()