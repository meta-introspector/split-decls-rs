macro_rules! deps {
    () => {
        Body!();
        Whitespace!();
        Event!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < 'a > Whitespace < 'a > { fn key_value_separators (& self) -> Vec < Event < 'a > > { let mut out = Vec :: with_capacity (3) ; if let Some (ws) = & self . pre_sep { out . push (Event :: Whitespace (ws . clone ())) ; } out . push (Event :: KeyValueSeparator) ; if let Some (ws) = & self . post_sep { out . push (Event :: Whitespace (ws . clone ())) ; } out } fn from_body (s : & file :: section :: Body < 'a >) -> Self { let key_pos = s . 0 . iter () . enumerate () . find_map (| (idx , e) | matches ! (e , Event :: SectionValueName (_)) . then (| | idx)) ; key_pos . map (| key_pos | { let pre_key = s . 0 [.. key_pos] . iter () . next_back () . and_then (| e | match e { Event :: Whitespace (s) => Some (s . clone ()) , _ => None , }) ; let from_key = & s . 0 [key_pos ..] ; let (pre_sep , post_sep) = from_key . iter () . enumerate () . find_map (| (idx , e) | matches ! (e , Event :: KeyValueSeparator) . then (| | idx)) . map (| sep_pos | { (from_key . get (sep_pos - 1) . and_then (| e | match e { Event :: Whitespace (ws) => Some (ws . clone ()) , _ => None , }) , from_key . get (sep_pos + 1) . and_then (| e | match e { Event :: Whitespace (ws) => Some (ws . clone ()) , _ => None , }) ,) }) . unwrap_or_default () ; Whitespace { pre_key , pre_sep , post_sep , } }) . unwrap_or_default () } }
    };
}

impl_16!();