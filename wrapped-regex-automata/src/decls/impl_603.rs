macro_rules! deps {
    () => {
        ByteSetIter!();
    };
}

macro_rules! impl_603 {
    () => {
        deps!();
        impl < 'a > Iterator for ByteSetIter < 'a > { type Item = u8 ; fn next (& mut self) -> Option < u8 > { while self . b <= 255 { let b = u8 :: try_from (self . b) . unwrap () ; self . b += 1 ; if self . set . contains (b) { return Some (b) ; } } None } }
    };
}

impl_603!()