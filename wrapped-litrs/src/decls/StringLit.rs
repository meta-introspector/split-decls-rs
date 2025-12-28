macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! StringLit {
    () => {
        deps!();
        # [doc = " A string or raw string literal, e.g. `\"foo\"`, `\"Grüße\"` or `r#\"a🦊c\"d🦀f\"#`."] # [doc = ""] # [doc = " See [the reference][ref] for more information."] # [doc = ""] # [doc = " [ref]: https://doc.rust-lang.org/reference/tokens.html#string-literals"] # [derive (Debug , Clone , PartialEq , Eq)] pub struct StringLit < B : Buffer > { # [doc = " The raw input."] raw : B , # [doc = " The string value (with all escapes unescaped), or `None` if there were"] # [doc = " no escapes. In the latter case, the string value is in `raw`."] value : Option < String > , # [doc = " The number of hash signs in case of a raw string literal, or `None` if"] # [doc = " it's not a raw string literal."] num_hashes : Option < u8 > , # [doc = " Start index of the suffix or `raw.len()` if there is no suffix."] start_suffix : usize , }
    };
}

StringLit!()