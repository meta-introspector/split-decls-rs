macro_rules! deps {
    () => {
        Item!();
        ReadRef!();
        DynamicRelocationIterator!();
        DynamicRelocationIteratorInternal!();
        Relocation!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > > Iterator for DynamicRelocationIterator < 'data , 'file , R > { type Item = (u64 , Relocation) ; fn next (& mut self) -> Option < Self :: Item > { match self . inner { # [cfg (feature = "elf")] DynamicRelocationIteratorInternal :: Elf32 (ref mut elf) => elf . next () , # [cfg (feature = "elf")] DynamicRelocationIteratorInternal :: Elf64 (ref mut elf) => elf . next () , DynamicRelocationIteratorInternal :: None (_) => None , } } }
    };
}

impl_168!();