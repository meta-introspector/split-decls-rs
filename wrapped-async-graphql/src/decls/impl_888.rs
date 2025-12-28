macro_rules! deps {
    () => {
        InputValueError!();
        InputValueResult!();
        ScalarType!();
        Scalar!();
    };
}

macro_rules! impl_888 {
    () => {
        deps!();
        # [doc = " The `Int` scalar type represents non-fractional whole numeric values."] # [Scalar (internal , name = "Int")] impl ScalarType for NonZeroU8 { fn parse (value : Value) -> InputValueResult < Self > { match value { Value :: Number (n) => { let n = n . as_u64 () . ok_or_else (| | InputValueError :: from ("Invalid number")) ? ; if n > u8 :: MAX as u64 || n == 0 { return Err (InputValueError :: from (format ! ("Only integers from {} to {} or non zero are accepted." , 1 , u8 :: MAX))) ; } Ok (NonZeroU8 :: new (n as u8) . unwrap ()) } _ => Err (InputValueError :: expected_type (value)) , } } fn is_valid (value : & Value) -> bool { matches ! (value , Value :: Number (n) if n . is_i64 ()) } fn to_value (& self) -> Value { Value :: Number (Number :: from (self . get () as u64)) } }
    };
}

impl_888!();