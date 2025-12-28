macro_rules! deps {
    () => {
        SymbolMap!();
    };
}

macro_rules! SymbolMapName {
    () => {
        deps!();
        # [doc = " The type used for entries in a [`SymbolMap`] that maps from addresses to names."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct SymbolMapName < 'data > { address : u64 , name : & 'data str , }
    };
}

SymbolMapName!()