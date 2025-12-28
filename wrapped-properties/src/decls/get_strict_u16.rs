macro_rules! get_strict_u16 {
    () => {
        # [doc = " Avoid monomorphizing multiple copies of this function"] fn get_strict_u16 (payload : & PropertyValueNameToEnumMap < '_ > , name : & str) -> Option < u16 > { payload . map . get (name) . and_then (| i | i . try_into () . ok ()) }
    };
}

get_strict_u16!();