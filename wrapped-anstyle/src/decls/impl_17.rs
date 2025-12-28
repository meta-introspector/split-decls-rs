macro_rules! deps {
    () => {
        RgbColor!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl From < (u8 , u8 , u8) > for RgbColor { # [inline] fn from (inner : (u8 , u8 , u8)) -> Self { let (r , g , b) = inner ; Self (r , g , b) } }
    };
}

impl_17!()