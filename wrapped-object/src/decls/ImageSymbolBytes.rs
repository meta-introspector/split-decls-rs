macro_rules! ImageSymbolBytes {
    () => {
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageSymbolBytes (pub [u8 ; IMAGE_SIZEOF_SYMBOL]) ;
    };
}

ImageSymbolBytes!();