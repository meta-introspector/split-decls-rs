macro_rules! deps {
    () => {
        SymbolId!();
    };
}

macro_rules! DynamicSymbolId {
    () => {
        deps!();
        # [doc = " A dynamic symbol ID."] pub type DynamicSymbolId = SymbolId < true > ;
    };
}

DynamicSymbolId!();