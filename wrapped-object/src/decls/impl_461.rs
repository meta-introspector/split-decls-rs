macro_rules! deps {
    () => {
        Item!();
        Result!();
        AttributeIndexIterator!();
    };
}

macro_rules! impl_461 {
    () => {
        deps!();
        impl < 'data > Iterator for AttributeIndexIterator < 'data > { type Item = Result < u32 > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
    };
}

impl_461!()