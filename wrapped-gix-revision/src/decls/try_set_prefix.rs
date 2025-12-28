macro_rules! deps {
    () => {
        Delegate!();
        PrefixHint!();
    };
}

macro_rules! try_set_prefix {
    () => {
        deps!();
        fn try_set_prefix (delegate : & mut impl Delegate , hex_name : & BStr , hint : Option < delegate :: PrefixHint < '_ > >) -> Option < () > { gix_hash :: Prefix :: from_hex (hex_name . to_str () . expect ("hexadecimal only")) . ok () . and_then (| prefix | delegate . disambiguate_prefix (prefix , hint)) }
    };
}

try_set_prefix!()