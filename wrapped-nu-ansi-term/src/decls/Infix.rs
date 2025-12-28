macro_rules! deps {
    () => {
        AnsiString!();
        Style!();
    };
}

macro_rules! Infix {
    () => {
        deps!();
        # [doc = " Like `AnsiString`, but only displays the difference between two"] # [doc = " styles."] # [doc = ""] # [doc = " This type implements the `Display` trait, meaning it can be written to a"] # [doc = " `std::fmt` formatting without doing any extra allocation, and written to a"] # [doc = " string with the `.to_string()` method. For examples, see"] # [doc = " [`Style::infix`](struct.Style.html#method.infix)."] # [derive (Clone , Copy , Debug)] pub struct Infix (Style , Style) ;
    };
}

Infix!();