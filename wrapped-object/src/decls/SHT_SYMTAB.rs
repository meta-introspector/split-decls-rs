macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! SHT_SYMTAB {
    () => {
        deps!();
        # [doc = " Symbol table."] pub const SHT_SYMTAB : u32 = 2 ;
    };
}

SHT_SYMTAB!();