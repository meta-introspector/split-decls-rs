macro_rules! deps {
    () => {
        Endian!();
        Relocation!();
        I32!();
        U32!();
    };
}

macro_rules! Rela32 {
    () => {
        deps!();
        # [doc = " Relocation table entry with explicit addend."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Rela32 < E : Endian > { # [doc = " Relocation address."] pub r_offset : U32 < E > , # [doc = " Relocation type and symbol index."] pub r_info : U32 < E > , # [doc = " Explicit addend."] pub r_addend : I32 < E > , }
    };
}

Rela32!()