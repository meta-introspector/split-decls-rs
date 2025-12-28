macro_rules! deps {
    () => {
        Symbol!();
        SymbolId!();
    };
}

macro_rules! impl_1134 {
    () => {
        deps!();
        impl < 'data , const DYNAMIC : bool > Symbol < 'data , DYNAMIC > { # [doc = " The ID used for referring to this symbol."] pub fn id (& self) -> SymbolId < DYNAMIC > { self . id } # [doc = " Get the `st_bind` component of the `st_info` field."] # [inline] pub fn st_bind (& self) -> u8 { self . st_info >> 4 } # [doc = " Get the `st_type` component of the `st_info` field."] # [inline] pub fn st_type (& self) -> u8 { self . st_info & 0xf } # [doc = " Set the `st_info` field given the `st_bind` and `st_type` components."] # [inline] pub fn set_st_info (& mut self , st_bind : u8 , st_type : u8) { self . st_info = (st_bind << 4) + (st_type & 0xf) ; } }
    };
}

impl_1134!()