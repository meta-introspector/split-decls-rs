macro_rules! deps {
    () => {
        U32!();
        Section!();
    };
}

macro_rules! DwarfAux32 {
    () => {
        deps!();
        # [doc = " Section auxiliary entry Format for C_DWARF symbols."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct DwarfAux32 { # [doc = " Length of portion of section represented by symbol."] pub x_scnlen : U32 < BE > , # [doc = " Reserved."] pub pad : [u8 ; 4] , # [doc = " Number of relocation entries in section."] pub x_nreloc : U32 < BE > , # [doc = " Reserved."] pub pad2 : [u8 ; 6] , }
    };
}

DwarfAux32!();