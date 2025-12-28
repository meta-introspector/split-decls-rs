macro_rules! deps {
    () => {
        Prefixes!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < 'a > Iterator for Prefixes < 'a > { type Item = & 'a str ; fn next (& mut self) -> Option < & 'a str > { self . iter . next () . map (| (i , ch) | & self . s [.. i + ch . len_utf8 ()]) } }
    };
}

impl_126!()