macro_rules! deps {
    () => {
        SymbolIteratorInternal!();
        Item!();
        SymbolIterator!();
        Symbol!();
        ReadRef!();
        SymbolInternal!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > > Iterator for SymbolIterator < 'data , 'file , R > { type Item = Symbol < 'data , 'file , R > ; fn next (& mut self) -> Option < Self :: Item > { map_inner_option_mut ! (self . inner , SymbolIteratorInternal , SymbolInternal , | iter | { iter . 0 . next () . map (| x | (x , PhantomData)) }) . map (| inner | Symbol { inner }) } }
    };
}

impl_160!()