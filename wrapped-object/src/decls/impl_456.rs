macro_rules! deps {
    () => {
        Item!();
        FileHeader!();
        Result!();
        AttributesSubsubsection!();
        AttributesSubsubsectionIterator!();
    };
}

macro_rules! impl_456 {
    () => {
        deps!();
        impl < 'data , Elf : FileHeader > Iterator for AttributesSubsubsectionIterator < 'data , Elf > { type Item = Result < AttributesSubsubsection < 'data > > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
    };
}

impl_456!();