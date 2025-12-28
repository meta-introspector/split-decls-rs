macro_rules! deps {
    () => {
        BorrowedCowStrDeserializer!();
        Error!();
        Result!();
        Deserializer!();
        MapKeyDeserializer!();
        Value!();
    };
}

macro_rules! impl_283 {
    () => {
        deps!();
        impl < 'de > serde :: Deserializer < 'de > for MapKeyDeserializer < 'de > { type Error = Error ; fn deserialize_any < V > (self , visitor : V) -> Result < V :: Value , Error > where V : Visitor < 'de > , { BorrowedCowStrDeserializer :: new (self . key) . deserialize_any (visitor) } deserialize_numeric_key ! (deserialize_i8) ; deserialize_numeric_key ! (deserialize_i16) ; deserialize_numeric_key ! (deserialize_i32) ; deserialize_numeric_key ! (deserialize_i64) ; deserialize_numeric_key ! (deserialize_u8) ; deserialize_numeric_key ! (deserialize_u16) ; deserialize_numeric_key ! (deserialize_u32) ; deserialize_numeric_key ! (deserialize_u64) ; # [cfg (not (feature = "float_roundtrip"))] deserialize_numeric_key ! (deserialize_f32) ; deserialize_numeric_key ! (deserialize_f64) ; # [cfg (feature = "float_roundtrip")] deserialize_numeric_key ! (deserialize_f32 , do_deserialize_f32) ; deserialize_numeric_key ! (deserialize_i128 , do_deserialize_i128) ; deserialize_numeric_key ! (deserialize_u128 , do_deserialize_u128) ; fn deserialize_bool < V > (self , visitor : V) -> Result < V :: Value , Error > where V : Visitor < 'de > , { if self . key == "true" { visitor . visit_bool (true) } else if self . key == "false" { visitor . visit_bool (false) } else { Err (serde :: de :: Error :: invalid_type (Unexpected :: Str (& self . key) , & visitor ,)) } } # [inline] fn deserialize_option < V > (self , visitor : V) -> Result < V :: Value , Error > where V : Visitor < 'de > , { visitor . visit_some (self) } # [inline] fn deserialize_newtype_struct < V > (self , _name : & 'static str , visitor : V ,) -> Result < V :: Value , Error > where V : Visitor < 'de > , { visitor . visit_newtype_struct (self) } fn deserialize_enum < V > (self , name : & 'static str , variants : & 'static [& 'static str] , visitor : V ,) -> Result < V :: Value , Error > where V : Visitor < 'de > , { self . key . into_deserializer () . deserialize_enum (name , variants , visitor) } forward_to_deserialize_any ! { char str string bytes byte_buf unit unit_struct seq tuple tuple_struct map struct identifier ignored_any } }
    };
}

impl_283!();