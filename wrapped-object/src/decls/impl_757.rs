macro_rules! deps {
    () => {
        Item!();
        WasmComdatIterator!();
        WasmComdat!();
    };
}

macro_rules! impl_757 {
    () => {
        deps!();
        impl < 'data , 'file , R > Iterator for WasmComdatIterator < 'data , 'file , R > { type Item = WasmComdat < 'data , 'file , R > ; # [inline] fn next (& mut self) -> Option < Self :: Item > { None } }
    };
}

impl_757!()