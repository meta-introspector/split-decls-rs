macro_rules! deps {
    () => {
        BorrowedCowStrDeserializer!();
        Deserializer!();
        Error!();
        Result!();
        Value!();
    };
}

macro_rules! impl_291 {
    () => {
        deps!();
        impl < 'de > de :: Deserializer < 'de > for BorrowedCowStrDeserializer < 'de > { type Error = Error ; fn deserialize_any < V > (self , visitor : V) -> Result < V :: Value , Error > where V : de :: Visitor < 'de > , { match self . value { Cow :: Borrowed (string) => visitor . visit_borrowed_str (string) , # [cfg (any (feature = "std" , feature = "alloc"))] Cow :: Owned (string) => visitor . visit_string (string) , # [cfg (not (any (feature = "std" , feature = "alloc")))] Cow :: Owned (_) => unreachable ! () , } } fn deserialize_enum < V > (self , _name : & str , _variants : & 'static [& 'static str] , visitor : V ,) -> Result < V :: Value , Error > where V : de :: Visitor < 'de > , { visitor . visit_enum (self) } forward_to_deserialize_any ! { bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map struct identifier ignored_any } }
    };
}

impl_291!();