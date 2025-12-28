macro_rules! deps {
    () => {
        Byte!();
    };
}

macro_rules! impl_288 {
    () => {
        deps!();
        # [cfg (test)] impl Byte for u8 { fn to_char (self) -> char { assert ! (self . is_ascii ()) ; char :: from (self) } }
    };
}

impl_288!();