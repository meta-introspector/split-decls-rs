macro_rules! deps {
    () => {
        Interest!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl ops :: BitOr for Interest { type Output = Self ; # [inline] fn bitor (self , other : Self) -> Self { self . add (other) } }
    };
}

impl_19!()