macro_rules! deps {
    () => {
        Scalar!();
        InputValueResult!();
        InputValueError!();
        ScalarType!();
    };
}

macro_rules! impl_822 {
    () => {
        deps!();
        # [doc = " The `Binary` scalar type represents binary data."] # [Scalar (internal)] impl ScalarType for Bytes { fn parse (value : Value) -> InputValueResult < Self > { match value { Value :: Binary (data) => Ok (data) , _ => Err (InputValueError :: expected_type (value)) , } } fn is_valid (value : & Value) -> bool { matches ! (value , Value :: Binary (_)) } fn to_value (& self) -> Value { Value :: Binary (self . clone ()) } }
    };
}

impl_822!()