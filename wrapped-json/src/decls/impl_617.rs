macro_rules! deps {
    () => {
        ReferenceFromString!();
        MapAccess!();
        Formatter!();
        Error!();
        Map!();
        RawKey!();
        Deserializer!();
        Value!();
        RawValue!();
        Result!();
    };
}

macro_rules! impl_617 {
    () => {
        deps!();
        impl < 'de : 'a , 'a > Deserialize < 'de > for & 'a RawValue { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { struct ReferenceVisitor ; impl < 'de > Visitor < 'de > for ReferenceVisitor { type Value = & 'de RawValue ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { write ! (formatter , "any valid JSON value") } fn visit_map < V > (self , mut visitor : V) -> Result < Self :: Value , V :: Error > where V : MapAccess < 'de > , { let value = tri ! (visitor . next_key ::< RawKey > ()) ; if value . is_none () { return Err (de :: Error :: invalid_type (Unexpected :: Map , & self)) ; } visitor . next_value_seed (ReferenceFromString) } } deserializer . deserialize_newtype_struct (TOKEN , ReferenceVisitor) } }
    };
}

impl_617!()