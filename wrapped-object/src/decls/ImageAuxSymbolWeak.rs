macro_rules! deps {
    () => {
        U32Bytes!();
        ImageSymbolEx!();
        ImageSymbol!();
    };
}

macro_rules! ImageAuxSymbolWeak {
    () => {
        deps!();
        # [doc = " Auxiliary symbol format 3: weak externals."] # [doc = ""] # [doc = " Used for both `ImageSymbol` and `ImageSymbolEx` (both with padding)."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageAuxSymbolWeak { # [doc = " the weak extern default symbol index"] pub weak_default_sym_index : U32Bytes < LE > , pub weak_search_type : U32Bytes < LE > , }
    };
}

ImageAuxSymbolWeak!();