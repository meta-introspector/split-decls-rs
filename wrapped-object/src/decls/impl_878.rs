macro_rules! deps {
    () => {
        NoDynamicRelocationIterator!();
        Item!();
        Relocation!();
    };
}

macro_rules! impl_878 {
    () => {
        deps!();
        impl Iterator for NoDynamicRelocationIterator { type Item = (u64 , Relocation) ; # [inline] fn next (& mut self) -> Option < Self :: Item > { None } }
    };
}

impl_878!();