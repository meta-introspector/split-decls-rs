macro_rules! deps {
    () => {
        Item!();
        WasmRelocationIterator!();
        Relocation!();
    };
}

macro_rules! impl_773 {
    () => {
        deps!();
        impl < 'data , 'file , R > Iterator for WasmRelocationIterator < 'data , 'file , R > { type Item = (u64 , Relocation) ; # [inline] fn next (& mut self) -> Option < Self :: Item > { None } }
    };
}

impl_773!()