macro_rules! deps {
    () => {
        Scalar!();
        InputValueError!();
        InputValueResult!();
        ScalarType!();
    };
}

macro_rules! impl_946 {
    () => {
        deps!();
        # [doc = " A local datetime without timezone offset."] # [doc = ""] # [doc = " The input/output is a string in ISO 8601 format without timezone, including"] # [doc = " subseconds. E.g. \"2022-01-12T07:30:19.12345\"."] # [Scalar (internal , name = "LocalDateTime")] impl ScalarType for PrimitiveDateTime { fn parse (value : Value) -> InputValueResult < Self > { match & value { Value :: String (s) => Ok (Self :: parse (s , & PRIMITIVE_DATE_TIME_FORMAT) ?) , _ => Err (InputValueError :: expected_type (value)) , } } fn to_value (& self) -> Value { Value :: String (self . format (& PRIMITIVE_DATE_TIME_FORMAT) . unwrap_or_else (| e | panic ! ("Failed to format `PrimitiveDateTime`: {}" , e)) ,) } }
    };
}

impl_946!()