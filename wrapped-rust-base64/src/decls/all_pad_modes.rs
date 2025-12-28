macro_rules! deps {
    () => {
        DecodePaddingMode!();
    };
}

macro_rules! all_pad_modes {
    () => {
        deps!();
        fn all_pad_modes () -> Vec < DecodePaddingMode > { vec ! [DecodePaddingMode :: Indifferent , DecodePaddingMode :: RequireCanonical , DecodePaddingMode :: RequireNone ,] }
    };
}

all_pad_modes!();