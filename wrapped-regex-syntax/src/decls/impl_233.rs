macro_rules! deps {
    () => {
        ClassUnicodeIter!();
        ClassUnicodeRange!();
    };
}

macro_rules! impl_233 {
    () => {
        deps!();
        impl < 'a > Iterator for ClassUnicodeIter < 'a > { type Item = & 'a ClassUnicodeRange ; fn next (& mut self) -> Option < & 'a ClassUnicodeRange > { self . 0 . next () } }
    };
}

impl_233!()