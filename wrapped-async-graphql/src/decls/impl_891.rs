macro_rules! deps {
    () => {
        InputValueError!();
        InputValueResult!();
        Scalar!();
        ScalarType!();
    };
}

macro_rules! impl_891 {
    () => {
        deps!();
        # [doc = " The `Int` scalar type represents non-fractional whole numeric values."] # [Scalar (internal , name = "Int")] impl ScalarType for NonZeroU64 { fn parse (value : Value) -> InputValueResult < Self > { match value { Value :: Number (n) => { let n = n . as_u64 () . ok_or_else (| | InputValueError :: from ("Invalid number")) ? ; if n == 0 { return Err (InputValueError :: from ("Only non zero are accepted.")) ; } Ok (NonZeroU64 :: new (n) . unwrap ()) } _ => Err (InputValueError :: expected_type (value)) , } } fn is_valid (value : & Value) -> bool { matches ! (value , Value :: Number (n) if n . is_i64 ()) } fn to_value (& self) -> Value { Value :: Number (Number :: from (self . get ())) } }
    };
}

impl_891!();