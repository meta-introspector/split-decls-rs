macro_rules! deps {
    () => {
        Item!();
        SectionIndex!();
        WasmComdatSectionIterator!();
    };
}

macro_rules! impl_762 {
    () => {
        deps!();
        impl < 'data , 'file , R > Iterator for WasmComdatSectionIterator < 'data , 'file , R > { type Item = SectionIndex ; fn next (& mut self) -> Option < Self :: Item > { None } }
    };
}

impl_762!();