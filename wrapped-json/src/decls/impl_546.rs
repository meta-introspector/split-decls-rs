macro_rules! deps {
    () => {
        Deserializer!();
        Result!();
        Value!();
        NumberKey!();
        Formatter!();
        Error!();
    };
}

macro_rules! impl_546 {
    () => {
        deps!();
        # [cfg (feature = "arbitrary_precision")] impl < 'de > de :: Deserialize < 'de > for NumberKey { fn deserialize < D > (deserializer : D) -> Result < NumberKey , D :: Error > where D : de :: Deserializer < 'de > , { struct FieldVisitor ; impl < 'de > de :: Visitor < 'de > for FieldVisitor { type Value = () ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a valid number field") } fn visit_str < E > (self , s : & str) -> Result < () , E > where E : de :: Error , { if s == TOKEN { Ok (()) } else { Err (de :: Error :: custom ("expected field with custom name")) } } } tri ! (deserializer . deserialize_identifier (FieldVisitor)) ; Ok (NumberKey) } }
    };
}

impl_546!()