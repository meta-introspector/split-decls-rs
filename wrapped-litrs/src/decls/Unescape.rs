macro_rules! Unescape {
    () => {
        # [doc = " Result of unescaping an escape-sequence in a string."] pub (crate) enum Unescape { Byte (u8) , Unicode (char) , }
    };
}

Unescape!()