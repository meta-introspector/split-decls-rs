macro_rules! deps {
    () => {
        I16!();
        U32!();
        Section!();
        U16!();
        U64!();
        Symbol!();
    };
}

macro_rules! Symbol64 {
    () => {
        deps!();
        # [doc = " Symbol table entry."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Symbol64 { # [doc = " Symbol value; storage class-dependent."] pub n_value : U64 < BE > , # [doc = " Offset of the name in string table or .debug section."] pub n_offset : U32 < BE > , # [doc = " Section number of symbol."] pub n_scnum : I16 < BE > , # [doc = " Basic and derived type specification."] pub n_type : U16 < BE > , # [doc = " Storage class of symbol."] pub n_sclass : u8 , # [doc = " Number of auxiliary entries."] pub n_numaux : u8 , }
    };
}

Symbol64!();