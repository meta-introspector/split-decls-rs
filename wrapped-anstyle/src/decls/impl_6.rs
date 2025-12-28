macro_rules! deps {
    () => {
        RgbColor!();
        Color!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl From < RgbColor > for Color { # [inline] fn from (inner : RgbColor) -> Self { Self :: Rgb (inner) } }
    };
}

impl_6!()