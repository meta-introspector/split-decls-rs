macro_rules! deps {
    () => {
        AnsiGenericStrings!();
        AnsiString!();
    };
}

macro_rules! AnsiStrings {
    () => {
        deps!();
        # [doc = " A function to construct an `AnsiStrings` instance."] # [allow (non_snake_case)] pub const fn AnsiStrings < 'a > (arg : & 'a [AnsiString < 'a >]) -> AnsiStrings < 'a > { AnsiGenericStrings (arg) }
    };
}

AnsiStrings!();