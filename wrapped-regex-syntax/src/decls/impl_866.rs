macro_rules! deps {
    () => {
        Utf8Range!();
    };
}

macro_rules! impl_866 {
    () => {
        deps!();
        impl Utf8Range { fn new (start : u8 , end : u8) -> Self { Utf8Range { start , end } } # [doc = " Returns true if and only if the given byte is in this range."] pub fn matches (& self , b : u8) -> bool { self . start <= b && b <= self . end } }
    };
}

impl_866!();