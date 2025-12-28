macro_rules! deps {
    () => {
        Round!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl Neg for Round { type Output = Round ; # [inline] fn neg (self) -> Round { match self { Round :: TowardPositive => Round :: TowardNegative , Round :: TowardNegative => Round :: TowardPositive , Round :: NearestTiesToEven | Round :: TowardZero | Round :: NearestTiesToAway => self , } } }
    };
}

impl_10!();