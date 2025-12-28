macro_rules! deps {
    () => {
        Symbol!();
        U32!();
        Relocation!();
    };
}

macro_rules! Rel32 {
    () => {
        deps!();
        # [doc = " Relocation table entry"] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Rel32 { # [doc = " Virtual address (position) in section to be relocated."] pub r_vaddr : U32 < BE > , # [doc = " Symbol table index of item that is referenced."] pub r_symndx : U32 < BE > , # [doc = " Relocation size and information."] pub r_rsize : u8 , # [doc = " Relocation type."] pub r_rtype : u8 , }
    };
}

Rel32!();