macro_rules! deps {
    () => {
        SymbolIteratorInternal!();
    };
}

macro_rules! ArchiveSymbolIterator {
    () => {
        deps!();
        # [doc = " An iterator over the symbols in the archive symbol table."] # [derive (Debug , Clone)] pub struct ArchiveSymbolIterator < 'data > (SymbolIteratorInternal < 'data >) ;
    };
}

ArchiveSymbolIterator!()