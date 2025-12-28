macro_rules! deps {
    () => {
        Relocation!();
        Item!();
        PeRelocationIterator!();
    };
}

macro_rules! impl_679 {
    () => {
        deps!();
        impl < 'data , 'file , R > Iterator for PeRelocationIterator < 'data , 'file , R > { type Item = (u64 , Relocation) ; fn next (& mut self) -> Option < Self :: Item > { None } }
    };
}

impl_679!()