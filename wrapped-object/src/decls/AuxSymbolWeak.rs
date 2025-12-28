macro_rules! AuxSymbolWeak {
    () => {
        # [doc = " Native endian version of [`pe::ImageAuxSymbolWeak`]."] # [allow (missing_docs)] # [derive (Debug , Default , Clone)] pub struct AuxSymbolWeak { pub weak_default_sym_index : u32 , pub weak_search_type : u32 , }
    };
}

AuxSymbolWeak!()