// Generated macro for impl_49 (impl)
macro_rules! Depcrateimpl_49 {
() => {
// Module: crate
// Provides: {"impl_49"}
// Dependencies: {}
impl < L , R > Either < Option < L > , Option < R > > { # [doc = " Factors out `None` from an `Either` of [`Option`]."] # [doc = ""] # [doc = " ```"] # [doc = " use either::*;"] # [doc = " let left: Either<_, Option<String>> = Left(Some(vec![0]));"] # [doc = " assert_eq!(left.factor_none(), Some(Left(vec![0])));"] # [doc = ""] # [doc = " let right: Either<Option<Vec<u8>>, _> = Right(Some(String::new()));"] # [doc = " assert_eq!(right.factor_none(), Some(Right(String::new())));"] # [doc = " ```"] # [doc (alias = "transpose")] pub fn factor_none (self) -> Option < Either < L , R > > { match self { Left (l) => l . map (Either :: Left) , Right (r) => r . map (Either :: Right) , } } }
};
}
