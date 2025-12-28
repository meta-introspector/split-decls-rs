macro_rules! deps {
    () => {
        U16Bytes!();
        U32Bytes!();
    };
}

macro_rules! ImageRelocation {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageRelocation { # [doc = " Also `RelocCount` when IMAGE_SCN_LNK_NRELOC_OVFL is set"] pub virtual_address : U32Bytes < LE > , pub symbol_table_index : U32Bytes < LE > , pub typ : U16Bytes < LE > , }
    };
}

ImageRelocation!();