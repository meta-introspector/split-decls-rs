macro_rules! deps {
    () => {
        SymbolMap!();
        SymbolMapEntry!();
    };
}

macro_rules! impl_908 {
    () => {
        deps!();
        impl < T : SymbolMapEntry > SymbolMap < T > { # [doc = " Construct a new symbol map."] # [doc = ""] # [doc = " This function will sort the symbols by address."] pub fn new (mut symbols : Vec < T >) -> Self { symbols . sort_by_key (| s | s . address ()) ; SymbolMap { symbols } } # [doc = " Get the symbol before the given address."] pub fn get (& self , address : u64) -> Option < & T > { let index = match self . symbols . binary_search_by_key (& address , | symbol | symbol . address ()) { Ok (index) => index , Err (index) => index . checked_sub (1) ? , } ; self . symbols . get (index) } # [doc = " Get all symbols in the map."] # [inline] pub fn symbols (& self) -> & [T] { & self . symbols } }
    };
}

impl_908!()