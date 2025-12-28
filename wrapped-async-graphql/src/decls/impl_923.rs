macro_rules! deps {
    () => {
        InputValueResult!();
        InputValueError!();
        ScalarType!();
        Scalar!();
    };
}

macro_rules! impl_923 {
    () => {
        deps!();
        # [doc = " Implement the DateTime<Utc> scalar"] # [doc = ""] # [doc = " The input/output is a string in RFC3339 format."] # [Scalar (internal , name = "DateTime" , specified_by_url = "https://datatracker.ietf.org/doc/html/rfc3339")] impl ScalarType for DateTime < Utc > { fn parse (value : Value) -> InputValueResult < Self > { match & value { Value :: String (s) => Ok (s . parse :: < DateTime < Utc > > () ?) , _ => Err (InputValueError :: expected_type (value)) , } } fn to_value (& self) -> Value { Value :: String (self . to_rfc3339 ()) } }
    };
}

impl_923!();