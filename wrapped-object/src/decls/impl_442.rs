macro_rules! deps {
    () => {
        Item!();
        FileHeader!();
        Endian!();
        Result!();
        VernauxIterator!();
        Vernaux!();
    };
}

macro_rules! impl_442 {
    () => {
        deps!();
        impl < 'data , Elf : FileHeader > Iterator for VernauxIterator < 'data , Elf > { type Item = Result < & 'data elf :: Vernaux < Elf :: Endian > > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
    };
}

impl_442!()