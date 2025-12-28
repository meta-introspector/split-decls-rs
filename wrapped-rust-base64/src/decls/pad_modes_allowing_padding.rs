macro_rules! deps {
    () => {
        DecodePaddingMode!();
    };
}

macro_rules! pad_modes_allowing_padding {
    () => {
        deps!();
        fn pad_modes_allowing_padding () -> Vec < DecodePaddingMode > { vec ! [DecodePaddingMode :: Indifferent , DecodePaddingMode :: RequireCanonical ,] }
    };
}

pad_modes_allowing_padding!();