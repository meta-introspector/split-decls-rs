macro_rules! deps {
    () => {
        RelocationBlockIterator!();
        RelocationIterator!();
        Item!();
        Result!();
    };
}

macro_rules! impl_713 {
    () => {
        deps!();
        impl < 'data > Iterator for RelocationBlockIterator < 'data > { type Item = Result < RelocationIterator < 'data > > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
    };
}

impl_713!();