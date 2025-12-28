macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < T , L , R > Either < Result < T , L > , Result < T , R > > { # [doc = " Factors out a homogeneous type from an `Either` of [`Result`]."] # [doc = ""] # [doc = " Here, the homogeneous type is the `Ok` type of the [`Result`]."] # [doc = ""] # [doc = " ```"] # [doc = " use either::*;"] # [doc = " let left: Either<_, Result<u32, String>> = Left(Err(vec![0]));"] # [doc = " assert_eq!(left.factor_ok(), Err(Left(vec![0])));"] # [doc = ""] # [doc = " let right: Either<Result<u32, Vec<u8>>, _> = Right(Err(String::new()));"] # [doc = " assert_eq!(right.factor_ok(), Err(Right(String::new())));"] # [doc = " ```"] # [doc (alias = "transpose")] pub fn factor_ok (self) -> Result < T , Either < L , R > > { match self { Left (l) => l . map_err (Either :: Left) , Right (r) => r . map_err (Either :: Right) , } } }
    };
}

impl_34!()