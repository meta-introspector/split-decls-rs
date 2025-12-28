macro_rules! deps {
    () => {
        Result!();
        Error!();
        Value!();
        UnitOnly!();
        BorrowedCowStrDeserializer!();
    };
}

macro_rules! impl_292 {
    () => {
        deps!();
        impl < 'de > de :: EnumAccess < 'de > for BorrowedCowStrDeserializer < 'de > { type Error = Error ; type Variant = UnitOnly ; fn variant_seed < T > (self , seed : T) -> Result < (T :: Value , Self :: Variant) , Error > where T : de :: DeserializeSeed < 'de > , { let value = tri ! (seed . deserialize (self)) ; Ok ((value , UnitOnly)) } }
    };
}

impl_292!()