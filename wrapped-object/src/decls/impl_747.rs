macro_rules! deps {
    () => {
        WasmSegmentIterator!();
        WasmSegment!();
        Item!();
    };
}

macro_rules! impl_747 {
    () => {
        deps!();
        impl < 'data , 'file , R > Iterator for WasmSegmentIterator < 'data , 'file , R > { type Item = WasmSegment < 'data , 'file , R > ; # [inline] fn next (& mut self) -> Option < Self :: Item > { None } }
    };
}

impl_747!()