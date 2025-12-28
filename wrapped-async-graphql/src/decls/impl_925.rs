macro_rules! deps {
    () => {
        InputValueError!();
        InputValueResult!();
        Scalar!();
        ScalarType!();
    };
}

macro_rules! impl_925 {
    () => {
        deps!();
        # [Scalar (internal , name = "Decimal")] impl ScalarType for Decimal { fn parse (value : Value) -> InputValueResult < Self > { match & value { Value :: String (s) => Ok (Decimal :: from_str (s) ?) , Value :: Number (n) => { if let Some (f) = n . as_f64 () { return Decimal :: try_from (f) . map_err (InputValueError :: custom) ; } if let Some (f) = n . as_i64 () { return Ok (Decimal :: from (f)) ; } Ok (Decimal :: from (n . as_u64 () . unwrap ())) } _ => Err (InputValueError :: expected_type (value)) , } } fn to_value (& self) -> Value { Value :: String (self . to_string ()) } }
    };
}

impl_925!()