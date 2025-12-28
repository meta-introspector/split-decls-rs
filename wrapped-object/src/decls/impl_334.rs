macro_rules! deps {
    () => {
        FileHeader!();
        Item!();
        ReadRef!();
        ElfSymbolIterator!();
        ElfSymbol!();
    };
}

macro_rules! impl_334 {
    () => {
        deps!();
        impl < 'data , 'file , Elf : FileHeader , R : ReadRef < 'data > > Iterator for ElfSymbolIterator < 'data , 'file , Elf , R > { type Item = ElfSymbol < 'data , 'file , Elf , R > ; fn next (& mut self) -> Option < Self :: Item > { let index = self . index ; let symbol = self . symbols . symbols . get (index . 0) ? ; self . index . 0 += 1 ; Some (ElfSymbol { endian : self . endian , symbols : self . symbols , index , symbol , }) } }
    };
}

impl_334!();