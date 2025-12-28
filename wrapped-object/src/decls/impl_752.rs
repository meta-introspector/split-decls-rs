macro_rules! deps {
    () => {
        Item!();
        WasmSection!();
        WasmSectionIterator!();
    };
}

macro_rules! impl_752 {
    () => {
        deps!();
        impl < 'data , 'file , R > Iterator for WasmSectionIterator < 'data , 'file , R > { type Item = WasmSection < 'data , 'file , R > ; fn next (& mut self) -> Option < Self :: Item > { let section = self . sections . next () ? ; Some (WasmSection { file : self . file , section , }) } }
    };
}

impl_752!()