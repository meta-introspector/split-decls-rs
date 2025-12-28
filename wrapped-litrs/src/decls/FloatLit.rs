macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! FloatLit {
    () => {
        deps!();
        # [doc = " A floating point literal, e.g. `3.14`, `8.`, `135e12`, or `1.956e2f64`."] # [doc = ""] # [doc = " This kind of literal has several forms, but generally consists of a main"] # [doc = " number part, an optional exponent and an optional type suffix. See"] # [doc = " [the reference][ref] for more information."] # [doc = ""] # [doc = " A leading minus sign `-` is not part of the literal grammar! `-3.14` are two"] # [doc = " tokens in the Rust grammar. Further, `27` and `27f32` are both not float,"] # [doc = " but integer literals! Consequently `FloatLit::parse` will reject them."] # [doc = ""] # [doc = ""] # [doc = " [ref]: https://doc.rust-lang.org/reference/tokens.html#floating-point-literals"] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct FloatLit < B : Buffer > { # [doc = " The whole raw input. The `usize` fields in this struct partition this"] # [doc = " string. Always true: `end_integer_part <= end_fractional_part`."] # [doc = ""] # [doc = " ```text"] # [doc = "    12_3.4_56e789f32"] # [doc = "        ╷    ╷   ╷"] # [doc = "        |    |   └ end_number_part = 13"] # [doc = "        |    └ end_fractional_part = 9"] # [doc = "        └ end_integer_part = 4"] # [doc = ""] # [doc = "    246."] # [doc = "       ╷╷"] # [doc = "       |└ end_fractional_part = end_number_part = 4"] # [doc = "       └ end_integer_part = 3"] # [doc = ""] # [doc = "    1234e89"] # [doc = "        ╷  ╷"] # [doc = "        |  └ end_number_part = 7"] # [doc = "        └ end_integer_part = end_fractional_part = 4"] # [doc = " ```"] raw : B , # [doc = " The first index not part of the integer part anymore. Since the integer"] # [doc = " part is at the start, this is also the length of that part."] end_integer_part : usize , # [doc = " The first index after the fractional part."] end_fractional_part : usize , # [doc = " The first index after the whole number part (everything except type suffix)."] end_number_part : usize , }
    };
}

FloatLit!()