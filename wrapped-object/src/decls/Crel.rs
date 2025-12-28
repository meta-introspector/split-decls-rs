macro_rules! deps {
    () => {
        Relocation!();
    };
}

macro_rules! Crel {
    () => {
        deps!();
        # [doc = " Compact relocation"] # [doc = ""] # [doc = " The specification has been submited here: <https://groups.google.com/g/generic-abi/c/ppkaxtLb0P0/m/awgqZ_1CBAAJ>."] # [derive (Debug , Clone , Copy)] pub struct Crel { # [doc = " Relocation offset."] pub r_offset : u64 , # [doc = " Relocation symbol index."] pub r_sym : u32 , # [doc = " Relocation type."] pub r_type : u32 , # [doc = " Relocation addend."] # [doc = ""] # [doc = " Only set if `CrelIterator::is_rela()` returns `true`."] pub r_addend : i64 , }
    };
}

Crel!();