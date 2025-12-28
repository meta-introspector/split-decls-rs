macro_rules! deps {
    () => {
        Lines!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < 'a > Iterator for Lines < 'a > { type Item = & 'a str ; fn next (& mut self) -> Option < Self :: Item > { self . 0 . next () . map (| it | unsafe { from_utf8_unchecked (it) }) } }
    };
}

impl_67!()