macro_rules! deps {
    () => {
        Result!();
        ExportsTrieIterator!();
        ExportSymbol!();
        Item!();
    };
}

macro_rules! impl_509 {
    () => {
        deps!();
        impl < 'data > Iterator for ExportsTrieIterator < 'data > { type Item = Result < ExportSymbol < 'data > > ; fn next (& mut self) -> Option < Self :: Item > { self . next () . transpose () } }
    };
}

impl_509!();