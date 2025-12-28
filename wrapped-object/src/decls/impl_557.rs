macro_rules! deps {
    () => {
        LoadCommandData!();
        Result!();
        Item!();
        Endian!();
        LoadCommandIterator!();
    };
}

macro_rules! impl_557 {
    () => {
        deps!();
        impl < 'data , E : Endian > Iterator for LoadCommandIterator < 'data , E > { type Item = Result < LoadCommandData < 'data , E > > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
    };
}

impl_557!();