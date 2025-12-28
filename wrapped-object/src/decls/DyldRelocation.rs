macro_rules! deps {
    () => {
        DyldRelocationAuth!();
    };
}

macro_rules! DyldRelocation {
    () => {
        deps!();
        # [doc = " A cache mapping relocation."] pub struct DyldRelocation { # [doc = " The offset of the relocation within the mapping."] # [doc = ""] # [doc = " This can be added to either the mapping file offset or the"] # [doc = " mapping address."] pub offset : u64 , # [doc = " The value to be relocated."] pub value : u64 , # [doc = " The pointer authentication data, if present."] pub auth : Option < DyldRelocationAuth > , }
    };
}

DyldRelocation!()