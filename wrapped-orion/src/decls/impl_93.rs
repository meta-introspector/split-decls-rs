macro_rules! deps {
    () => {
        StreamTag!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl StreamTag { # [inline] # [doc = " Return the tag as a byte."] pub fn as_byte (& self) -> u8 { match * self { StreamTag :: Message => 0b0000_0000 , StreamTag :: Push => 0b0000_0001 , StreamTag :: Rekey => 0b0000_0010 , StreamTag :: Finish => 0b0000_0011 , } } }
    };
}

impl_93!()