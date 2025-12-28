macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl < 'a > Iterator for Iter < 'a > { type Item = Option < & 'a str > ; fn next (& mut self) -> Option < Option < & 'a str > > { self . range . next () . map (| i | self . arr . get (i)) } fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } }
    };
}

impl_155!()