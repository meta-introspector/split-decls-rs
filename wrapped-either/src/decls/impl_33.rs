macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < L , R , E > Either < Result < L , E > , Result < R , E > > { # [doc = " Factors out a homogeneous type from an `Either` of [`Result`]."] # [doc = ""] # [doc = " Here, the homogeneous type is the `Err` type of the [`Result`]."] # [doc = ""] # [doc = " ```"] # [doc = " use either::*;"] # [doc = " let left: Either<_, Result<String, u32>> = Left(Ok(vec![0]));"] # [doc = " assert_eq!(left.factor_err(), Ok(Left(vec![0])));"] # [doc = ""] # [doc = " let right: Either<Result<Vec<u8>, u32>, _> = Right(Ok(String::new()));"] # [doc = " assert_eq!(right.factor_err(), Ok(Right(String::new())));"] # [doc = " ```"] # [doc (alias = "transpose")] pub fn factor_err (self) -> Result < Either < L , R > , E > { match self { Left (l) => l . map (Either :: Left) , Right (r) => r . map (Either :: Right) , } } }
    };
}

impl_33!()