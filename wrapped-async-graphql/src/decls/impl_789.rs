macro_rules! deps {
    () => {
        Result!();
        Error!();
        MaybeUndefined!();
    };
}

macro_rules! impl_789 {
    () => {
        deps!();
        impl < T : Serialize > Serialize for MaybeUndefined < T > { fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { match self { MaybeUndefined :: Value (value) => value . serialize (serializer) , _ => serializer . serialize_none () , } } }
    };
}

impl_789!();