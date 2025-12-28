macro_rules! deps {
    () => {
        Error!();
        Result!();
        CursorType!();
    };
}

macro_rules! impl_716 {
    () => {
        deps!();
        # [cfg (feature = "chrono")] impl CursorType for chrono :: DateTime < chrono :: Utc > { type Error = chrono :: ParseError ; fn decode_cursor (s : & str) -> Result < Self , Self :: Error > { Ok (chrono :: DateTime :: parse_from_rfc3339 (s) ? . with_timezone :: < chrono :: Utc > (& chrono :: Utc { })) } fn encode_cursor (& self) -> String { self . to_rfc3339_opts (chrono :: SecondsFormat :: Micros , true) } }
    };
}

impl_716!();