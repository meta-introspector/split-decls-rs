macro_rules! deps {
    () => {
        Relocation!();
        RelocationIterator!();
        Item!();
    };
}

macro_rules! impl_716 {
    () => {
        deps!();
        impl < 'data > Iterator for RelocationIterator < 'data > { type Item = Relocation ; fn next (& mut self) -> Option < Relocation > { loop { let reloc = self . relocs . next () ? . get (LE) ; if reloc != 0 { return Some (Relocation { virtual_address : self . virtual_address . wrapping_add ((reloc & 0xfff) as u32) , typ : reloc >> 12 , }) ; } } } }
    };
}

impl_716!()