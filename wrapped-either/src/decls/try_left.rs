macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! try_left {
    () => {
        deps!();
        # [doc = " Macro for unwrapping the left side of an [`Either`], which fails early"] # [doc = " with the opposite side. Can only be used in functions that return"] # [doc = " `Either` because of the early return of `Right` that it provides."] # [doc = ""] # [doc = " See also [`try_right!`] for its dual, which applies the same just to the"] # [doc = " right side."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use either::{Either, Left, Right};"] # [doc = ""] # [doc = " fn twice(wrapper: Either<u32, &str>) -> Either<u32, &str> {"] # [doc = "     let value = either::try_left!(wrapper);"] # [doc = "     Left(value * 2)"] # [doc = " }"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     assert_eq!(twice(Left(2)), Left(4));"] # [doc = "     assert_eq!(twice(Right(\"ups\")), Right(\"ups\"));"] # [doc = " }"] # [doc = " ```"] # [macro_export] macro_rules ! try_left { ($ expr : expr) => { match $ expr { $ crate :: Left (val) => val , $ crate :: Right (err) => return $ crate :: Right (:: core :: convert :: From :: from (err)) , } } ; }
    };
}

try_left!();