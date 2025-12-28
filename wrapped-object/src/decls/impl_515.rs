macro_rules! deps {
    () => {
        Result!();
        ExportSymbol!();
        Item!();
        NodeIterator!();
    };
}

macro_rules! impl_515 {
    () => {
        deps!();
        impl < 'data > Iterator for NodeIterator < 'data > { type Item = Result < Option < ExportSymbol < 'data > > > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
    };
}

impl_515!();