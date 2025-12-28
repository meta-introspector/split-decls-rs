macro_rules! ImageSymbolExBytes {
    () => {
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageSymbolExBytes (pub [u8 ; IMAGE_SIZEOF_SYMBOL_EX]) ;
    };
}

ImageSymbolExBytes!()