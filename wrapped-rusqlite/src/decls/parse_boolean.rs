macro_rules! parse_boolean {
    () => {
        # [doc = " The boolean can be one of:"] # [doc = " ```text"] # [doc = " 1 yes true on"] # [doc = " 0 no false off"] # [doc = " ```"] # [must_use] pub fn parse_boolean (s : & str) -> Option < bool > { if s . eq_ignore_ascii_case ("yes") || s . eq_ignore_ascii_case ("on") || s . eq_ignore_ascii_case ("true") || s . eq ("1") { Some (true) } else if s . eq_ignore_ascii_case ("no") || s . eq_ignore_ascii_case ("off") || s . eq_ignore_ascii_case ("false") || s . eq ("0") { Some (false) } else { None } }
    };
}

parse_boolean!();