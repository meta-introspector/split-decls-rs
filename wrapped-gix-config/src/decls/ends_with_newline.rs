macro_rules! deps {
    () => {
        Event!();
    };
}

macro_rules! ends_with_newline {
    () => {
        deps!();
        pub (crate) fn ends_with_newline (e : & [crate :: parse :: Event < '_ >] , nl : impl AsRef < [u8] > , default : bool) -> bool { if e . is_empty () { return default ; } e . iter () . rev () . take_while (| e | e . to_bstr_lossy () . iter () . all (u8 :: is_ascii_whitespace)) . find_map (| e | e . to_bstr_lossy () . contains_str (nl . as_ref ()) . then_some (true)) . unwrap_or (false) }
    };
}

ends_with_newline!();