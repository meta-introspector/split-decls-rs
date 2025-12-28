macro_rules! deps {
    () => {
        Result!();
        Item!();
        FunctionStartsIterator!();
    };
}

macro_rules! impl_553 {
    () => {
        deps!();
        impl < 'data > Iterator for FunctionStartsIterator < 'data > { type Item = Result < u64 > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
    };
}

impl_553!()