macro_rules! deps {
    () => {
        Error!();
        Time!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl FromStr for Time { type Err = Error ; fn from_str (s : & str) -> Result < Self , Self :: Err > { crate :: parse_header (s) . ok_or_else (| | Error :: InvalidDateString { input : s . into () }) } }
    };
}

impl_28!();