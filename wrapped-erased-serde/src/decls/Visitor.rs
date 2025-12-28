macro_rules! deps {
    () => {
        EnumAccess!();
        SeqAccess!();
        Deserializer!();
        Result!();
        MapAccess!();
        Out!();
        Error!();
    };
}

macro_rules! Visitor {
    () => {
        deps!();
        pub trait Visitor < 'de > { fn erased_expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result ; fn erased_visit_bool (& mut self , v : bool) -> Result < Out , Error > ; fn erased_visit_i8 (& mut self , v : i8) -> Result < Out , Error > ; fn erased_visit_i16 (& mut self , v : i16) -> Result < Out , Error > ; fn erased_visit_i32 (& mut self , v : i32) -> Result < Out , Error > ; fn erased_visit_i64 (& mut self , v : i64) -> Result < Out , Error > ; fn erased_visit_i128 (& mut self , v : i128) -> Result < Out , Error > ; fn erased_visit_u8 (& mut self , v : u8) -> Result < Out , Error > ; fn erased_visit_u16 (& mut self , v : u16) -> Result < Out , Error > ; fn erased_visit_u32 (& mut self , v : u32) -> Result < Out , Error > ; fn erased_visit_u64 (& mut self , v : u64) -> Result < Out , Error > ; fn erased_visit_u128 (& mut self , v : u128) -> Result < Out , Error > ; fn erased_visit_f32 (& mut self , v : f32) -> Result < Out , Error > ; fn erased_visit_f64 (& mut self , v : f64) -> Result < Out , Error > ; fn erased_visit_char (& mut self , v : char) -> Result < Out , Error > ; fn erased_visit_str (& mut self , v : & str) -> Result < Out , Error > ; fn erased_visit_borrowed_str (& mut self , v : & 'de str) -> Result < Out , Error > ; # [cfg (feature = "alloc")] fn erased_visit_string (& mut self , v : String) -> Result < Out , Error > ; fn erased_visit_bytes (& mut self , v : & [u8]) -> Result < Out , Error > ; fn erased_visit_borrowed_bytes (& mut self , v : & 'de [u8]) -> Result < Out , Error > ; # [cfg (feature = "alloc")] fn erased_visit_byte_buf (& mut self , v : Vec < u8 >) -> Result < Out , Error > ; fn erased_visit_none (& mut self) -> Result < Out , Error > ; fn erased_visit_some (& mut self , deserializer : & mut dyn Deserializer < 'de >) -> Result < Out , Error > ; fn erased_visit_unit (& mut self) -> Result < Out , Error > ; fn erased_visit_newtype_struct (& mut self , deserializer : & mut dyn Deserializer < 'de > ,) -> Result < Out , Error > ; fn erased_visit_seq (& mut self , seq : & mut dyn SeqAccess < 'de >) -> Result < Out , Error > ; fn erased_visit_map (& mut self , map : & mut dyn MapAccess < 'de >) -> Result < Out , Error > ; fn erased_visit_enum (& mut self , data : & mut dyn EnumAccess < 'de >) -> Result < Out , Error > ; }
    };
}

Visitor!()