macro_rules! deps {
    () => {
        Symbol!();
        U32!();
        Relocation!();
        U64!();
    };
}

macro_rules! Rel64 {
    () => {
        deps!();
        # [doc = " Relocation table entry"] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Rel64 { # [doc = " Virtual address (position) in section to be relocated."] pub r_vaddr : U64 < BE > , # [doc = " Symbol table index of item that is referenced."] pub r_symndx : U32 < BE > , # [doc = " Relocation size and information."] pub r_rsize : u8 , # [doc = " Relocation type."] pub r_rtype : u8 , }
    };
}

Rel64!()