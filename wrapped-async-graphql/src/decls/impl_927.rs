macro_rules! deps {
    () => {
        ScalarType!();
        InputValueResult!();
        InputValueError!();
        Scalar!();
    };
}

macro_rules! impl_927 {
    () => {
        deps!();
        # [doc = " Implement the Duration scalar"] # [doc = ""] # [doc = " The input/output is a string in ISO8601 format."] # [Scalar (internal , name = "Duration" , specified_by_url = "https://en.wikipedia.org/wiki/ISO_8601#Durations")] impl ScalarType for Duration { fn parse (value : Value) -> InputValueResult < Self > { match & value { Value :: String (s) => Ok (Duration :: from_std (std :: time :: Duration :: from (iso8601 :: Duration :: from_str (s) ? ,)) ?) , _ => Err (InputValueError :: expected_type (value)) , } } fn to_value (& self) -> Value { Value :: String (self . to_string ()) } }
    };
}

impl_927!();