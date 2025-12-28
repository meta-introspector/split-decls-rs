macro_rules! deps {
    () => {
        PeRelocationIterator!();
        Relocation!();
        Item!();
    };
}

macro_rules! impl_679 {
    () => {
        deps!();
        impl < 'data , 'file , R > Iterator for PeRelocationIterator < 'data , 'file , R > { type Item = (u64 , Relocation) ; fn next (& mut self) -> Option < Self :: Item > { None } }
    };
}

impl_679!();