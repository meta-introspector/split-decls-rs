macro_rules! deps {
    () => {
        FileHeader!();
        Endian!();
        Item!();
        VernauxIterator!();
        Result!();
        Verneed!();
        VerneedIterator!();
    };
}

macro_rules! impl_439 {
    () => {
        deps!();
        impl < 'data , Elf : FileHeader > Iterator for VerneedIterator < 'data , Elf > { type Item = Result < (& 'data elf :: Verneed < Elf :: Endian > , VernauxIterator < 'data , Elf > ,) > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
    };
}

impl_439!();