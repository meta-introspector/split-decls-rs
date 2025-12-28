macro_rules! deps {
    () => {
        Scalar!();
        InputValueError!();
        InputValueResult!();
        ScalarType!();
    };
}

macro_rules! impl_841 {
    () => {
        deps!();
        # [doc = " The `Int` scalar type represents non-fractional whole numeric values."] # [Scalar (internal , name = "Int")] impl ScalarType for isize { fn parse (value : Value) -> InputValueResult < Self > { match value { Value :: Number (n) => { let n = n . as_i64 () . ok_or_else (| | InputValueError :: from ("Invalid number")) ? ; if n < Self :: MIN as i64 || n > Self :: MAX as i64 { return Err (InputValueError :: from (format ! ("Only integers from {} to {} are accepted." , Self :: MIN , Self :: MAX))) ; } Ok (n as Self) } _ => Err (InputValueError :: expected_type (value)) , } } fn is_valid (value : & Value) -> bool { matches ! (value , Value :: Number (n) if n . is_i64 ()) } fn to_value (& self) -> Value { Value :: Number (Number :: from (* self as i64)) } }
    };
}

impl_841!();