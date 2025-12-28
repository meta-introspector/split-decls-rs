macro_rules! deps {
    () => {
        Scalar!();
        InputValueError!();
        ScalarType!();
        InputValueResult!();
    };
}

macro_rules! impl_942 {
    () => {
        deps!();
        # [doc = " A datetime with timezone offset."] # [doc = ""] # [doc = " The input is a string in RFC3339 format, e.g. \"2022-01-12T04:00:19.12345Z\""] # [doc = " or \"2022-01-12T04:00:19+03:00\". The output is also a string in RFC3339"] # [doc = " format, but it is always normalized to the UTC (Z) offset, e.g."] # [doc = " \"2022-01-12T04:00:19.12345Z\"."] # [Scalar (internal , name = "DateTime" , specified_by_url = "https://datatracker.ietf.org/doc/html/rfc3339")] impl ScalarType for OffsetDateTime { fn parse (value : Value) -> InputValueResult < Self > { match & value { Value :: String (s) => Ok (Self :: parse (s , & Rfc3339) ?) , _ => Err (InputValueError :: expected_type (value)) , } } fn to_value (& self) -> Value { Value :: String (self . to_offset (UtcOffset :: UTC) . format (& Rfc3339) . unwrap_or_else (| e | panic ! ("Failed to format `OffsetDateTime`: {}" , e)) ,) } }
    };
}

impl_942!();