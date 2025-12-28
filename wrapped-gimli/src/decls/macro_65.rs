macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! macro_65 {
    () => {
        deps!();
        dw ! (# [doc = " The child determination encodings for DIE attributes."] # [doc = ""] # [doc = " See Section 7.5.3, Table 7.4."] DwChildren (u8) { DW_CHILDREN_no = 0 , DW_CHILDREN_yes = 1 , }) ;
    };
}

macro_65!();