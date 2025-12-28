macro_rules! deps {
    () => {
        IterBytes!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl < 'a > DoubleEndedIterator for IterBytes < 'a > { fn next_back (& mut self) -> Option < & 'a [u8] > { self . range . next_back () . and_then (| i | self . arr . get_bytes (i)) } }
    };
}

impl_160!()