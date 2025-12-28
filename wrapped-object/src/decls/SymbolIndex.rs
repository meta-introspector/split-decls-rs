macro_rules! SymbolIndex {
    () => {
        # [doc = " The index of an ELF symbol."] # [derive (Debug , Default , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct SymbolIndex (pub u32) ;
    };
}

SymbolIndex!()