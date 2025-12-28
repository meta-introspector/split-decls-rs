macro_rules! deps {
    () => {
        IntegerBase!();
    };
}

macro_rules! impl_196 {
    () => {
        deps!();
        impl IntegerBase { # [doc = " Returns the literal prefix that indicates this base, i.e. `\"0b\"`,"] # [doc = " `\"0o\"`, `\"\"` and `\"0x\"`."] pub fn prefix (self) -> & 'static str { match self { Self :: Binary => "0b" , Self :: Octal => "0o" , Self :: Decimal => "" , Self :: Hexadecimal => "0x" , } } # [doc = " Returns the base value, i.e. 2, 8, 10 or 16."] pub fn value (self) -> u8 { match self { Self :: Binary => 2 , Self :: Octal => 8 , Self :: Decimal => 10 , Self :: Hexadecimal => 16 , } } }
    };
}

impl_196!()