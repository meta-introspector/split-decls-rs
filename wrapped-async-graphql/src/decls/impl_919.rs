macro_rules! deps {
    () => {
        InputValueError!();
        Scalar!();
        InputValueResult!();
        ScalarType!();
    };
}

macro_rules! impl_919 {
    () => {
        deps!();
        # [Scalar (internal , name = "TimeZone" , specified_by_url = "http://www.iana.org/time-zones")] impl ScalarType for Tz { fn parse (value : Value) -> InputValueResult < Self > { match value { Value :: String (s) => Ok (s . parse () ?) , _ => Err (InputValueError :: expected_type (value)) , } } fn to_value (& self) -> Value { Value :: String (self . name () . to_owned ()) } }
    };
}

impl_919!()