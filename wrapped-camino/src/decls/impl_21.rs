macro_rules! deps {
    () => {
        Utf8Components!();
        Utf8Component!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < 'a > Iterator for Utf8Components < 'a > { type Item = Utf8Component < 'a > ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . 0 . next () . map (| component | { unsafe { Utf8Component :: new (component) } }) } }
    };
}

impl_21!()