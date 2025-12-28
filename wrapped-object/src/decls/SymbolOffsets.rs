macro_rules! deps {
    () => {
        StringId!();
        SymbolId!();
    };
}

macro_rules! SymbolOffsets {
    () => {
        deps!();
        # [derive (Default , Clone , Copy)] struct SymbolOffsets { index : usize , str_id : Option < StringId > , aux_count : u8 , storage_class : u8 , x_smtyp : u8 , x_smclas : u8 , containing_csect : Option < SymbolId > , }
    };
}

SymbolOffsets!();