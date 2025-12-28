macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! ByteStringLit {
    () => {
        deps!();
        # [doc = " A byte string or raw byte string literal, e.g. `b\"hello\"` or `br#\"abc\"def\"#`."] # [doc = ""] # [doc = " See [the reference][ref] for more information."] # [doc = ""] # [doc = " [ref]: https://doc.rust-lang.org/reference/tokens.html#byte-string-literals"] # [derive (Debug , Clone , PartialEq , Eq)] pub struct ByteStringLit < B : Buffer > { # [doc = " The raw input."] raw : B , # [doc = " The string value (with all escaped unescaped), or `None` if there were"] # [doc = " no escapes. In the latter case, `input` is the string value."] value : Option < Vec < u8 > > , # [doc = " The number of hash signs in case of a raw string literal, or `None` if"] # [doc = " it's not a raw string literal."] num_hashes : Option < u8 > , # [doc = " Start index of the suffix or `raw.len()` if there is no suffix."] start_suffix : usize , }
    };
}

ByteStringLit!()