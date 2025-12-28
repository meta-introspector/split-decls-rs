macro_rules! deps {
    () => {
        U32!();
        Section!();
        I16!();
        U16!();
        Symbol!();
    };
}

macro_rules! Symbol32 {
    () => {
        deps!();
        # [doc = " Symbol table entry."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Symbol32 { # [doc = " Symbol name."] # [doc = ""] # [doc = " If first 4 bytes are 0, then second 4 bytes are offset into string table."] pub n_name : [u8 ; 8] , # [doc = " Symbol value; storage class-dependent."] pub n_value : U32 < BE > , # [doc = " Section number of symbol."] pub n_scnum : I16 < BE > , # [doc = " Basic and derived type specification."] pub n_type : U16 < BE > , # [doc = " Storage class of symbol."] pub n_sclass : u8 , # [doc = " Number of auxiliary entries."] pub n_numaux : u8 , }
    };
}

Symbol32!();