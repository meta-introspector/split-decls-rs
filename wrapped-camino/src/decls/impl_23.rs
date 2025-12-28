macro_rules! deps {
    () => {
        Utf8Components!();
        Utf8Component!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl DoubleEndedIterator for Utf8Components < '_ > { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { self . 0 . next_back () . map (| component | { unsafe { Utf8Component :: new (component) } }) } }
    };
}

impl_23!()