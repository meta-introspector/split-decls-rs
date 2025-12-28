macro_rules! deps {
    () => {
        Color!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl From < (u8 , u8 , u8) > for Color { # [inline] fn from (inner : (u8 , u8 , u8)) -> Self { Self :: Rgb (inner . into ()) } }
    };
}

impl_8!();