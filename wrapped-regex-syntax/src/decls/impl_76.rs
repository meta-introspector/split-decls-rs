macro_rules! deps {
    () => {
        HexLiteralKind!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl HexLiteralKind { # [doc = " The number of digits that must be used with this literal form when"] # [doc = " used without brackets. When used with brackets, there is no"] # [doc = " restriction on the number of digits."] pub fn digits (& self) -> u32 { match * self { HexLiteralKind :: X => 2 , HexLiteralKind :: UnicodeShort => 4 , HexLiteralKind :: UnicodeLong => 8 , } } }
    };
}

impl_76!();