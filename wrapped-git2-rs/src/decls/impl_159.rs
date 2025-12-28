macro_rules! deps {
    () => {
        IterBytes!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl < 'a > Iterator for IterBytes < 'a > { type Item = & 'a [u8] ; fn next (& mut self) -> Option < & 'a [u8] > { self . range . next () . and_then (| i | self . arr . get_bytes (i)) } fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } }
    };
}

impl_159!();