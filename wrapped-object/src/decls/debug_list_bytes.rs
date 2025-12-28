macro_rules! deps {
    () => {
        DebugLen!();
        Result!();
        DebugByte!();
    };
}

macro_rules! debug_list_bytes {
    () => {
        deps!();
        fn debug_list_bytes (bytes : & [u8] , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut list = fmt . debug_list () ; list . entries (bytes . iter () . take (8) . copied () . map (DebugByte)) ; if bytes . len () > 8 { list . entry (& DebugLen (bytes . len ())) ; } list . finish () }
    };
}

debug_list_bytes!();