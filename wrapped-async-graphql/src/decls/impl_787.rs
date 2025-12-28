macro_rules! deps {
    () => {
        InputValueResult!();
        InputValueError!();
        Registry!();
        InputType!();
        MaybeUndefined!();
    };
}

macro_rules! impl_787 {
    () => {
        deps!();
        impl < T : InputType > InputType for MaybeUndefined < T > { type RawValueType = T :: RawValueType ; fn type_name () -> Cow < 'static , str > { T :: type_name () } fn qualified_type_name () -> String { T :: type_name () . to_string () } fn create_type_info (registry : & mut registry :: Registry) -> String { T :: create_type_info (registry) ; T :: type_name () . to_string () } fn parse (value : Option < Value >) -> InputValueResult < Self > { match value { None => Ok (MaybeUndefined :: Undefined) , Some (Value :: Null) => Ok (MaybeUndefined :: Null) , Some (value) => Ok (MaybeUndefined :: Value (T :: parse (Some (value)) . map_err (InputValueError :: propagate) ? ,)) , } } fn to_value (& self) -> Value { match self { MaybeUndefined :: Value (value) => value . to_value () , _ => Value :: Null , } } fn as_raw_value (& self) -> Option < & Self :: RawValueType > { if let MaybeUndefined :: Value (value) = self { value . as_raw_value () } else { None } } }
    };
}

impl_787!();