macro_rules! deps {
    () => {
        ClassBytesIter!();
        ClassBytesRange!();
    };
}

macro_rules! impl_241 {
    () => {
        deps!();
        impl < 'a > Iterator for ClassBytesIter < 'a > { type Item = & 'a ClassBytesRange ; fn next (& mut self) -> Option < & 'a ClassBytesRange > { self . 0 . next () } }
    };
}

impl_241!();