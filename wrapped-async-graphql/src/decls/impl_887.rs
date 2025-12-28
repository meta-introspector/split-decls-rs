macro_rules! deps {
    () => {
        Scalar!();
        InputValueResult!();
        InputValueError!();
        ScalarType!();
    };
}

macro_rules! impl_887 {
    () => {
        deps!();
        # [doc = " The `Int` scalar type represents non-fractional whole numeric values."] # [Scalar (internal , name = "Int")] impl ScalarType for NonZeroIsize { fn parse (value : Value) -> InputValueResult < Self > { match value { Value :: Number (n) => { let n = n . as_i64 () . ok_or_else (| | InputValueError :: from ("Invalid number")) ? ; if n < isize :: MIN as i64 || n > isize :: MAX as i64 || n == 0 { return Err (InputValueError :: from (format ! ("Only integers from {} to {} or non zero are accepted." , isize :: MIN , isize :: MAX))) ; } Ok (NonZeroIsize :: new (n as isize) . unwrap ()) } _ => Err (InputValueError :: expected_type (value)) , } } fn is_valid (value : & Value) -> bool { matches ! (value , Value :: Number (n) if n . is_i64 ()) } fn to_value (& self) -> Value { Value :: Number (Number :: from (self . get () as i64)) } }
    };
}

impl_887!();