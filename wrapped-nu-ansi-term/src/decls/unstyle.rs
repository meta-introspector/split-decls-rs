macro_rules! deps {
    () => {
        AnsiStrings!();
    };
}

macro_rules! unstyle {
    () => {
        deps!();
        # [doc = " Return a concatenated copy of `strs` without the formatting, as an allocated `String`."] pub fn unstyle (strs : & AnsiStrings) -> String { let mut s = String :: new () ; for i in strs . 0 . iter () { s += i . string . deref () ; } s }
    };
}

unstyle!()