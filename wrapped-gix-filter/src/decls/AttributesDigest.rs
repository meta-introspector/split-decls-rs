macro_rules! AttributesDigest {
    () => {
        # [doc = " The combination of `crlf`, `text` and `eol` attributes into one neat package."] # [derive (Debug , Copy , Clone , Eq , PartialEq)] pub enum AttributesDigest { # [doc = " Equivalent to the `-text` attribute."] Binary , # [doc = " Equivalent to the `text` attribute."] Text , # [doc = " Equivalent to the `text eol=lf` attributes."] TextInput , # [doc = " Equivalent to the `text eol=crlf` attributes."] TextCrlf , # [doc = " Equivalent to the `text=auto` attributes."] TextAuto , # [doc = " Equivalent to the `text=auto eol=crlf` attributes."] TextAutoCrlf , # [doc = " Equivalent to the `text=auto eol=lf` attributes."] TextAutoInput , }
    };
}

AttributesDigest!()