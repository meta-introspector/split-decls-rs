macro_rules! deps {
    () => {
        AnsiByteString!();
        AnsiGenericStrings!();
    };
}

macro_rules! AnsiByteStrings {
    () => {
        deps!();
        # [doc = " A function to construct an `AnsiByteStrings` instance."] # [allow (non_snake_case)] pub const fn AnsiByteStrings < 'a > (arg : & 'a [AnsiByteString < 'a >]) -> AnsiByteStrings < 'a > { AnsiGenericStrings (arg) }
    };
}

AnsiByteStrings!();