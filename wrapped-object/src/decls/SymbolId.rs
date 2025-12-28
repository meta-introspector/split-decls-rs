macro_rules! deps {
    () => {
        Symbols!();
    };
}

macro_rules! SymbolId {
    () => {
        deps!();
        # [doc = " An ID for referring to a symbol in [`Symbols`]."] # [derive (Clone , Copy , PartialEq , Eq)] pub struct SymbolId < const DYNAMIC : bool = false > (usize) ;
    };
}

SymbolId!();