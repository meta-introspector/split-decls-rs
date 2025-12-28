macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < T > DoubleEndedIterator for IterMut < '_ , T > { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { self . inner . next_back () } }
    };
}

impl_33!()