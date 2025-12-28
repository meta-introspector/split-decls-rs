macro_rules! deps {
    () => {
        DateTime!();
        DateTimeVisitor!();
        Error!();
        Local!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        # [doc = " Deserialize an RFC 3339 formatted string into a `DateTime<Local>`"] # [doc = ""] # [doc = " The value will remain the same instant in UTC, but the offset will be recalculated to match"] # [doc = " that of the `Local` platform time zone."] # [doc = ""] # [doc = " As an extension to RFC 3339 this can deserialize to `DateTime`s outside the range of 0-9999"] # [doc = " years using an ISO 8601 syntax (which prepends an `-` or `+`)."] # [doc = ""] # [doc = " See [the `serde` module](crate::serde) for alternate deserialization formats."] # [cfg (feature = "clock")] impl < 'de > de :: Deserialize < 'de > for DateTime < Local > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { deserializer . deserialize_str (DateTimeVisitor) . map (| dt | dt . with_timezone (& Local)) } }
    };
}

impl_68!()