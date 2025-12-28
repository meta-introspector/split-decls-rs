macro_rules! deps {
    () => {
        InputScheme!();
        Url!();
    };
}

macro_rules! find_scheme {
    () => {
        deps!();
        pub (crate) fn find_scheme (input : & BStr) -> InputScheme { if let Some (protocol_end) = input . find ("://") { return InputScheme :: Url { protocol_end } ; } if let Some (colon) = input . find_byte (b':') { let explicitly_local = & input [.. colon] . contains (& b'/') ; let dos_driver_letter = cfg ! (windows) && input [.. colon] . len () == 1 ; if ! explicitly_local && ! dos_driver_letter { return InputScheme :: Scp { colon } ; } } InputScheme :: Local }
    };
}

find_scheme!()