macro_rules! deps {
    () => {
        VernauxIterator!();
        FileHeader!();
        Item!();
        VerneedIterator!();
        Result!();
        Verneed!();
        Endian!();
    };
}

macro_rules! impl_439 {
    () => {
        deps!();
        impl < 'data , Elf : FileHeader > Iterator for VerneedIterator < 'data , Elf > { type Item = Result < (& 'data elf :: Verneed < Elf :: Endian > , VernauxIterator < 'data , Elf > ,) > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
    };
}

impl_439!()