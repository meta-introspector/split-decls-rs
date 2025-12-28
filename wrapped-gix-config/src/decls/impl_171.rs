macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl < 'a > Key < 'a > { # [doc = " Parse `input` like `remote.origin` or `core` as a `Key` to make its section specific fields available,"] # [doc = " or `None` if there were not one or two tokens separated by `.`."] # [doc = " Note that `input` isn't validated, and is `str` as ascii is a subset of UTF-8 which is required for any valid keys."] pub fn parse (input : impl Into < & 'a BStr >) -> Option < Self > { let input = input . into () ; let mut tokens = input . splitn (2 , | b | * b == b'.') ; Some (Key { section_name : tokens . next () ? . to_str () . ok () ? , subsection_name : tokens . next () . map (Into :: into) , }) } }
    };
}

impl_171!()