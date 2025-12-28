macro_rules! SymbolIndex {
    () => {
        # [derive (Default)] pub struct SymbolIndex { symbols : Box < [FileSymbol] > , map : fst :: Map < Vec < u8 > > , }
    };
}

SymbolIndex!();