macro_rules! deps {
    () => {
        InputValueResult!();
        InputValueError!();
        Scalar!();
        ScalarType!();
    };
}

macro_rules! impl_915 {
    () => {
        deps!();
        # [Scalar (internal , name = "JSON")] impl ScalarType for Bson { fn parse (value : Value) -> InputValueResult < Self > { bson :: to_bson (& value) . map_err (InputValueError :: custom) } fn to_value (& self) -> Value { bson :: from_bson (self . clone ()) . unwrap_or_default () } }
    };
}

impl_915!();