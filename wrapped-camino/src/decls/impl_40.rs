macro_rules! deps {
    () => {
        Utf8Component!();
        Utf8Components!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl DoubleEndedIterator for Utf8Components < '_ > { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { self . 0 . next_back () . map (| component | { unsafe { Utf8Component :: new (component) } }) } }
    };
}

impl_40!();