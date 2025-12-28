macro_rules! deps {
    () => {
        ScalarType!();
        Scalar!();
        InputValueResult!();
        InputValueError!();
    };
}

macro_rules! impl_931 {
    () => {
        deps!();
        # [Scalar (internal)] # [doc = " ISO 8601 combined date and time without timezone."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " * `2015-07-01T08:59:60.123`,"] impl ScalarType for NaiveDateTime { fn parse (value : Value) -> InputValueResult < Self > { match value { Value :: String (s) => Ok (NaiveDateTime :: parse_from_str (& s , "%Y-%m-%dT%H:%M:%S%.f") ?) , _ => Err (InputValueError :: expected_type (value)) , } } fn to_value (& self) -> Value { Value :: String (self . format ("%Y-%m-%dT%H:%M:%S%.f") . to_string ()) } }
    };
}

impl_931!()