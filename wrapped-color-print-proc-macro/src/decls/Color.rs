macro_rules! deps {
    () => {
        Color256!();
        ColorRgb!();
        Color16!();
    };
}

macro_rules! Color {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Clone)] # [allow (clippy :: enum_variant_names)] pub enum Color { Color16 (Color16) , Color256 (Color256) , ColorRgb (ColorRgb) , }
    };
}

Color!()