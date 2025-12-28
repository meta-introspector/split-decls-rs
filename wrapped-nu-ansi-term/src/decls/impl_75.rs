macro_rules! deps {
    () => {
        Rgb!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl From < (u8 , u8 , u8) > for Rgb { fn from ((r , g , b) : (u8 , u8 , u8)) -> Self { Self :: new (r , g , b) } }
    };
}

impl_75!()