macro_rules! deps {
    () => {
        Utf8Path!();
        Utf8Ancestors!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < 'a > Iterator for Utf8Ancestors < 'a > { type Item = & 'a Utf8Path ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . 0 . next () . map (| path | { unsafe { Utf8Path :: assume_utf8 (path) } }) } }
    };
}

impl_17!()