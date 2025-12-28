macro_rules! deps {
    () => {
        U64!();
        Section!();
    };
}

macro_rules! DwarfAux64 {
    () => {
        deps!();
        # [doc = " Section auxiliary entry Format for C_DWARF symbols."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct DwarfAux64 { # [doc = " Length of portion of section represented by symbol."] pub x_scnlen : U64 < BE > , # [doc = " Number of relocation entries in section."] pub x_nreloc : U64 < BE > , # [doc = " Reserved."] pub pad : u8 , # [doc = " Contains _AUX_SECT; Type of Auxiliary entry."] pub x_auxtype : u8 , }
    };
}

DwarfAux64!()