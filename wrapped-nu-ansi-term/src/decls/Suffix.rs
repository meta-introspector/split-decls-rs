macro_rules! deps {
    () => {
        AnsiString!();
        Style!();
    };
}

macro_rules! Suffix {
    () => {
        deps!();
        # [doc = " Like `AnsiString`, but only displays the style suffix."] # [doc = ""] # [doc = " This type implements the `Display` trait, meaning it can be written to a"] # [doc = " `std::fmt` formatting without doing any extra allocation, and written to a"] # [doc = " string with the `.to_string()` method. For examples, see"] # [doc = " [`Style::suffix`](struct.Style.html#method.suffix)."] # [derive (Clone , Copy , Debug)] pub struct Suffix (Style) ;
    };
}

Suffix!();