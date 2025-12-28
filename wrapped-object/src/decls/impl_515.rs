macro_rules! deps {
    () => {
        Item!();
        Result!();
        ExportSymbol!();
        NodeIterator!();
    };
}

macro_rules! impl_515 {
    () => {
        deps!();
        impl < 'data > Iterator for NodeIterator < 'data > { type Item = Result < Option < ExportSymbol < 'data > > > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
    };
}

impl_515!()