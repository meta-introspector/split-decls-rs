macro_rules! deps {
    () => {
        NoteIterator!();
        Note!();
        Result!();
        FileHeader!();
        Item!();
    };
}

macro_rules! impl_407 {
    () => {
        deps!();
        impl < 'data , Elf : FileHeader > Iterator for NoteIterator < 'data , Elf > { type Item = read :: Result < Note < 'data , Elf > > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
    };
}

impl_407!();