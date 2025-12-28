macro_rules! deps {
    () => {
        Section!();
        U32!();
        Symbol!();
        U16!();
    };
}

macro_rules! StatAux {
    () => {
        deps!();
        # [doc = " Section auxiliary entry for the C_STAT Symbol. (XCOFF32 Only)"] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct StatAux { # [doc = " Section length."] pub x_scnlen : U32 < BE > , # [doc = " Number of relocation entries."] pub x_nreloc : U16 < BE > , # [doc = " Number of line numbers."] pub x_nlinno : U16 < BE > , # [doc = " Reserved."] pub pad : [u8 ; 10] , }
    };
}

StatAux!();