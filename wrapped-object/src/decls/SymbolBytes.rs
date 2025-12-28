macro_rules! SymbolBytes {
    () => {
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct SymbolBytes (pub [u8 ; SIZEOF_SYMBOL]) ;
    };
}

SymbolBytes!();