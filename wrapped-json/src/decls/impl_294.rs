macro_rules! deps {
    () => {
        Error!();
        Result!();
        VariantAccess!();
        Value!();
        UnitOnly!();
    };
}

macro_rules! impl_294 {
    () => {
        deps!();
        impl < 'de > de :: VariantAccess < 'de > for UnitOnly { type Error = Error ; fn unit_variant (self) -> Result < () , Error > { Ok (()) } fn newtype_variant_seed < T > (self , _seed : T) -> Result < T :: Value , Error > where T : de :: DeserializeSeed < 'de > , { Err (de :: Error :: invalid_type (Unexpected :: UnitVariant , & "newtype variant" ,)) } fn tuple_variant < V > (self , _len : usize , _visitor : V) -> Result < V :: Value , Error > where V : de :: Visitor < 'de > , { Err (de :: Error :: invalid_type (Unexpected :: UnitVariant , & "tuple variant" ,)) } fn struct_variant < V > (self , _fields : & 'static [& 'static str] , _visitor : V ,) -> Result < V :: Value , Error > where V : de :: Visitor < 'de > , { Err (de :: Error :: invalid_type (Unexpected :: UnitVariant , & "struct variant" ,)) } }
    };
}

impl_294!()