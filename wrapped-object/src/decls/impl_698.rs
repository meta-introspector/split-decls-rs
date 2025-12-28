macro_rules! deps {
    () => {
        Item!();
        Result!();
        ImageImportDescriptor!();
        ImportDescriptorIterator!();
    };
}

macro_rules! impl_698 {
    () => {
        deps!();
        impl < 'data > Iterator for ImportDescriptorIterator < 'data > { type Item = Result < & 'data pe :: ImageImportDescriptor > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
    };
}

impl_698!()