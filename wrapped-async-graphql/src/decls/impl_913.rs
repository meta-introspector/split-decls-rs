macro_rules! deps {
    () => {
        ScalarType!();
        InputValueResult!();
        InputValueError!();
        Object!();
        Scalar!();
    };
}

macro_rules! impl_913 {
    () => {
        deps!();
        # [Scalar (internal , name = "UUID")] impl ScalarType for Uuid { fn parse (value : Value) -> InputValueResult < Self > { match value { Value :: String (s) => Ok (Uuid :: parse_str (s) ?) , Value :: Object (o) => { let json = Value :: Object (o) . into_json () ? ; let Bson :: Binary (binary) = Bson :: try_from (json) ? else { return Err (InputValueError :: custom ("could not parse the value as BSON Binary" ,)) ; } ; binary . to_uuid () . map_err (| _ | { InputValueError :: custom ("could not deserialize BSON Binary to Uuid") }) } _ => Err (InputValueError :: expected_type (value)) , } } fn to_value (& self) -> Value { Value :: String (self . to_string ()) } }
    };
}

impl_913!()