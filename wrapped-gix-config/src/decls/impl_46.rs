macro_rules! deps {
    () => {
        File!();
        Error!();
        Metadata!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < 'a > TryFrom < & 'a str > for File < 'a > { type Error = parse :: Error ; # [doc = " Convenience constructor. Attempts to parse the provided string into a"] # [doc = " [`File`]. See [`Events::from_str()`][crate::parse::Events::from_str()] for more information."] fn try_from (s : & 'a str) -> Result < File < 'a > , Self :: Error > { parse :: Events :: from_str (s) . map (| events | Self :: from_parse_events_no_includes (events , Metadata :: api ())) } }
    };
}

impl_46!();