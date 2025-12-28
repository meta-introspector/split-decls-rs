macro_rules! deps {
    () => {
        Verdef!();
        VerdefIterator!();
        Result!();
        FileHeader!();
        Item!();
        Endian!();
        VerdauxIterator!();
    };
}

macro_rules! impl_433 {
    () => {
        deps!();
        impl < 'data , Elf : FileHeader > Iterator for VerdefIterator < 'data , Elf > { type Item = Result < (& 'data elf :: Verdef < Elf :: Endian > , VerdauxIterator < 'data , Elf >) > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
    };
}

impl_433!()