macro_rules! deps {
    () => {
        Item!();
        Result!();
        ImageDelayloadDescriptor!();
        DelayLoadDescriptorIterator!();
    };
}

macro_rules! impl_709 {
    () => {
        deps!();
        impl < 'data > Iterator for DelayLoadDescriptorIterator < 'data > { type Item = Result < & 'data pe :: ImageDelayloadDescriptor > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
    };
}

impl_709!();