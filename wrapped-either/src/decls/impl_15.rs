macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < T , L , R > Either < (T , L) , (T , R) > { # [doc = " Factor out a homogeneous type from an either of pairs."] # [doc = ""] # [doc = " Here, the homogeneous type is the first element of the pairs."] # [doc = ""] # [doc = " ```"] # [doc = " use either::*;"] # [doc = " let left: Either<_, (u32, String)> = Left((123, vec![0]));"] # [doc = " assert_eq!(left.factor_first().0, 123);"] # [doc = ""] # [doc = " let right: Either<(u32, Vec<u8>), _> = Right((123, String::new()));"] # [doc = " assert_eq!(right.factor_first().0, 123);"] # [doc = " ```"] pub fn factor_first (self) -> (T , Either < L , R >) { match self { Left ((t , l)) => (t , Left (l)) , Right ((t , r)) => (t , Right (r)) , } } }
    };
}

impl_15!()