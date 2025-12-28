macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! unescape_and_check_attr_value {
    () => {
        deps!();
        fn unescape_and_check_attr_value (value : & BStr) -> Result < BString , Error > { let mut out = BString :: from (Vec :: with_capacity (value . len ())) ; let mut bytes = value . iter () ; while let Some (mut b) = bytes . next () . copied () { if b == b'\\' { b = * bytes . next () . ok_or (Error :: TrailingEscapeCharacter) ? ; } out . push (validated_attr_value_byte (b) ?) ; } Ok (out) }
    };
}

unescape_and_check_attr_value!();