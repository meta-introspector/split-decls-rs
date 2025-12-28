macro_rules! deps {
    () => {
        InputValueResult!();
        Scalar!();
        ScalarType!();
        InputValueError!();
    };
}

macro_rules! impl_898 {
    () => {
        deps!();
        # [doc = " The `String` scalar type represents textual data, represented as UTF-8"] # [doc = " character sequences. The String type is most often used by GraphQL to"] # [doc = " represent free-form human-readable text."] # [Scalar (internal)] impl ScalarType for String { fn parse (value : Value) -> InputValueResult < Self > { match value { Value :: String (s) => Ok (s) , _ => Err (InputValueError :: expected_type (value)) , } } fn is_valid (value : & Value) -> bool { matches ! (value , Value :: String (_)) } fn to_value (& self) -> Value { Value :: String (self . clone ()) } }
    };
}

impl_898!()