macro_rules! deps {
    () => {
        Endian!();
        GnuProperty!();
        Result!();
        GnuPropertyIterator!();
        Item!();
    };
}

macro_rules! impl_415 {
    () => {
        deps!();
        impl < 'data , Endian : endian :: Endian > Iterator for GnuPropertyIterator < 'data , Endian > { type Item = read :: Result < GnuProperty < 'data > > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
    };
}

impl_415!()