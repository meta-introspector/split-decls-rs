macro_rules! deps {
    () => {
        File!();
        Metadata!();
        Error!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl FromStr for File < 'static > { type Err = parse :: Error ; fn from_str (s : & str) -> Result < Self , Self :: Err > { parse :: Events :: from_bytes_owned (s . as_bytes () , None) . map (| events | File :: from_parse_events_no_includes (events , Metadata :: api ())) } }
    };
}

impl_45!()