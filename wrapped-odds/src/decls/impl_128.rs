macro_rules! deps {
    () => {
        Suffixes!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl < 'a > Iterator for Suffixes < 'a > { type Item = & 'a str ; fn next (& mut self) -> Option < & 'a str > { self . iter . next () . map (| (i , _) | & self . s [i ..]) } }
    };
}

impl_128!();