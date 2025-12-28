macro_rules! deps {
    () => {
        InputValueResult!();
        InputValueError!();
        Scalar!();
        ScalarType!();
    };
}

macro_rules! impl_922 {
    () => {
        deps!();
        # [doc = " Implement the DateTime<Local> scalar"] # [doc = ""] # [doc = " The input/output is a string in RFC3339 format."] # [Scalar (internal , name = "DateTime" , specified_by_url = "https://datatracker.ietf.org/doc/html/rfc3339")] impl ScalarType for DateTime < Local > { fn parse (value : Value) -> InputValueResult < Self > { match & value { Value :: String (s) => Ok (s . parse :: < DateTime < Local > > () ?) , _ => Err (InputValueError :: expected_type (value)) , } } fn to_value (& self) -> Value { Value :: String (self . to_rfc3339 ()) } }
    };
}

impl_922!();