macro_rules! deps {
    () => {
        U16!();
        Endian!();
        U64!();
        Symbol!();
        U32!();
        Section!();
    };
}

macro_rules! Sym64 {
    () => {
        deps!();
        # [doc = " Symbol table entry."] # [derive (Debug , Default , Clone , Copy)] # [repr (C)] pub struct Sym64 < E : Endian > { # [doc = " Symbol name."] # [doc = ""] # [doc = " This is an offset into the symbol string table."] pub st_name : U32 < E > , # [doc = " Symbol type and binding."] # [doc = ""] # [doc = " Use the `st_bind` and `st_type` methods to access this value."] pub st_info : u8 , # [doc = " Symbol visibility."] # [doc = ""] # [doc = " Use the `st_visibility` method to access this value."] pub st_other : u8 , # [doc = " Section index or one of the `SHN_*` values."] pub st_shndx : U16 < E > , # [doc = " Symbol value."] pub st_value : U64 < E > , # [doc = " Symbol size."] pub st_size : U64 < E > , }
    };
}

Sym64!()