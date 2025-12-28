macro_rules! deps {
    () => {
        Metadata!();
        File!();
        Error!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < 'a > TryFrom < & 'a BStr > for File < 'a > { type Error = parse :: Error ; # [doc = " Convenience constructor. Attempts to parse the provided byte string into"] # [doc = " a [`File`]. See [`Events::from_bytes()`][parse::Events::from_bytes()] for more information."] fn try_from (value : & 'a BStr) -> Result < File < 'a > , Self :: Error > { parse :: Events :: from_bytes (value , None) . map (| events | Self :: from_parse_events_no_includes (events , Metadata :: api ())) } }
    };
}

impl_47!()