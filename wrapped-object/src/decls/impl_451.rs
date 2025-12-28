macro_rules! deps {
    () => {
        Result!();
        FileHeader!();
        AttributesSubsectionIterator!();
        Item!();
        AttributesSubsection!();
    };
}

macro_rules! impl_451 {
    () => {
        deps!();
        impl < 'data , Elf : FileHeader > Iterator for AttributesSubsectionIterator < 'data , Elf > { type Item = Result < AttributesSubsection < 'data , Elf > > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
    };
}

impl_451!()