macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < K > DoubleEndedIterator for Drain < '_ , K > { # [inline] fn next_back (& mut self) -> Option < K > { self . iter . next_back () . map (| (k , _) | k) } }
    };
}

impl_167!()