macro_rules! deps {
    () => {
        Relr!();
        RelrIterator!();
        Item!();
        FileHeader!();
    };
}

macro_rules! impl_369 {
    () => {
        deps!();
        impl < 'data , Elf : FileHeader > Iterator for RelrIterator < 'data , Elf > { type Item = Elf :: Word ; fn next (& mut self) -> Option < Self :: Item > { loop { while self . count > 0 { self . count -= 1 ; let offset = Elf :: Relr :: next (& mut self . offset , & mut self . bits) ; if offset . is_some () { return offset ; } } let next = self . iter . next () ? . get (self . endian) ; if next . into () & 1 == 0 { self . offset = next ; return Some (next) ; } self . bits = next ; self . count = Elf :: Relr :: COUNT ; } } }
    };
}

impl_369!()