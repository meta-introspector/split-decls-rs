macro_rules! deps {
    () => {
        InputValueResult!();
        ScalarType!();
        InputValueError!();
        Scalar!();
    };
}

macro_rules! impl_936 {
    () => {
        deps!();
        # [Scalar (internal , name = "SmolStr" , specified_by_url = "https://docs.rs/smol_str/latest/smol_str/struct.SmolStr.html")] impl ScalarType for SmolStr { fn parse (value : Value) -> InputValueResult < Self > { match value { Value :: String (s) => Ok (SmolStr :: new (s)) , _ => Err (InputValueError :: expected_type (value)) , } } fn is_valid (value : & Value) -> bool { matches ! (value , Value :: String (_)) } fn to_value (& self) -> Value { Value :: String (self . to_string ()) } }
    };
}

impl_936!();