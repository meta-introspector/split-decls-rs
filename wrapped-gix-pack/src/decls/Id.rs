macro_rules! Id {
    () => {
        # [doc = " An identifier to uniquely identify all packs loaded within a known context or namespace."] pub type Id = u32 ;
    };
}

Id!();