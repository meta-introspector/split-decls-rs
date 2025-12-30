// Generated macro for impl_53 (impl)
macro_rules! Depcrateimpl_53 {
() => {
// Module: crate
// Provides: {"impl_53"}
// Dependencies: {}
impl < T , L , R > Either < (L , T) , (R , T) > { # [doc = " Factor out a homogeneous type from an either of pairs."] # [doc = ""] # [doc = " Here, the homogeneous type is the second element of the pairs."] # [doc = ""] # [doc = " ```"] # [doc = " use either::*;"] # [doc = " let left: Either<_, (String, u32)> = Left((vec![0], 123));"] # [doc = " assert_eq!(left.factor_second().1, 123);"] # [doc = ""] # [doc = " let right: Either<(Vec<u8>, u32), _> = Right((String::new(), 123));"] # [doc = " assert_eq!(right.factor_second().1, 123);"] # [doc = " ```"] pub fn factor_second (self) -> (Either < L , R > , T) { match self { Left ((l , t)) => (Left (l) , t) , Right ((r , t)) => (Right (r) , t) , } } }
};
}
