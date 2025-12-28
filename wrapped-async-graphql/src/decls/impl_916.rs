macro_rules! deps {
    () => {
        Scalar!();
        ScalarType!();
        InputValueResult!();
        InputValueError!();
    };
}

macro_rules! impl_916 {
    () => {
        deps!();
        # [Scalar (internal , name = "JSONObject")] impl ScalarType for Document { fn parse (value : Value) -> InputValueResult < Self > { bson :: to_document (& value) . map_err (InputValueError :: custom) } fn to_value (& self) -> Value { bson :: from_document (self . clone ()) . unwrap_or_default () } }
    };
}

impl_916!();