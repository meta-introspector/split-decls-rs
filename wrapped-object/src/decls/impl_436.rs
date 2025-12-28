macro_rules! deps {
    () => {
        Result!();
        Endian!();
        VerdauxIterator!();
        Verdaux!();
        FileHeader!();
        Item!();
    };
}

macro_rules! impl_436 {
    () => {
        deps!();
        impl < 'data , Elf : FileHeader > Iterator for VerdauxIterator < 'data , Elf > { type Item = Result < & 'data elf :: Verdaux < Elf :: Endian > > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
    };
}

impl_436!()