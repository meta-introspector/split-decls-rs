macro_rules! deps {
    () => {
        Scalar!();
        InputValueResult!();
        InputValueError!();
        ScalarType!();
    };
}

macro_rules! impl_914 {
    () => {
        deps!();
        # [cfg (feature = "chrono")] # [Scalar (internal , name = "DateTime")] impl ScalarType for UtcDateTime { fn parse (value : Value) -> InputValueResult < Self > { < DateTime < Utc > > :: parse (value) . map_err (InputValueError :: propagate) . map (UtcDateTime :: from_chrono) } fn to_value (& self) -> Value { self . to_chrono () . to_value () } }
    };
}

impl_914!();