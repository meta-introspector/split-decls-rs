macro_rules! deps {
    () => {
        Substrings!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl < 'a > Iterator for Substrings < 'a > { type Item = & 'a str ; fn next (& mut self) -> Option < & 'a str > { self . iter . next () } }
    };
}

impl_130!()