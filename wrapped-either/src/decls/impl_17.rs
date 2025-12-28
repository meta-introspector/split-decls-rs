macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < T > Either < T , T > { # [doc = " Extract the value of an either over two equivalent types."] # [doc = ""] # [doc = " ```"] # [doc = " use either::*;"] # [doc = ""] # [doc = " let left: Either<_, u32> = Left(123);"] # [doc = " assert_eq!(left.into_inner(), 123);"] # [doc = ""] # [doc = " let right: Either<u32, _> = Right(123);"] # [doc = " assert_eq!(right.into_inner(), 123);"] # [doc = " ```"] pub fn into_inner (self) -> T { for_both ! (self , inner => inner) } # [doc = " Map `f` over the contained value and return the result in the"] # [doc = " corresponding variant."] # [doc = ""] # [doc = " ```"] # [doc = " use either::*;"] # [doc = ""] # [doc = " let value: Either<_, i32> = Right(42);"] # [doc = ""] # [doc = " let other = value.map(|x| x * 2);"] # [doc = " assert_eq!(other, Right(84));"] # [doc = " ```"] pub fn map < F , M > (self , f : F) -> Either < M , M > where F : FnOnce (T) -> M , { map_either ! (self , t => f (t)) } }
    };
}

impl_17!()