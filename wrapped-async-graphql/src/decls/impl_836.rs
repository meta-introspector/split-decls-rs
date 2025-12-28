macro_rules! deps {
    () => {
        InputValueError!();
        ScalarType!();
        Scalar!();
        InputValueResult!();
    };
}

macro_rules! impl_836 {
    () => {
        deps!();
        # [doc = " The `Int` scalar type represents non-fractional whole numeric values."] # [Scalar (internal , name = "Int")] impl ScalarType for u8 { fn parse (value : Value) -> InputValueResult < Self > { match value { Value :: Number (n) => { let n = n . as_u64 () . ok_or_else (| | InputValueError :: from ("Invalid number")) ? ; if n > Self :: MAX as u64 { return Err (InputValueError :: from (format ! ("Only integers from {} to {} are accepted." , 0 , Self :: MAX))) ; } Ok (n as Self) } _ => Err (InputValueError :: expected_type (value)) , } } fn is_valid (value : & Value) -> bool { matches ! (value , Value :: Number (n) if n . is_u64 ()) } fn to_value (& self) -> Value { Value :: Number (Number :: from (* self as u64)) } }
    };
}

impl_836!();