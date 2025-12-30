// Generated macro for for_both (macro)
macro_rules! Depcratefor_both {
() => {
// Module: crate
// Provides: {"for_both"}
// Dependencies: {}
# [doc = " Evaluate the provided expression for both [`Either::Left`] and [`Either::Right`]."] # [doc = ""] # [doc = " This macro is useful in cases where both sides of [`Either`] can be interacted with"] # [doc = " in the same way even though the don't share the same type."] # [doc = ""] # [doc = " Syntax: `either::for_both!(` *expression* `,` *pattern* `=>` *expression* `)`"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use either::Either;"] # [doc = ""] # [doc = " fn length(owned_or_borrowed: Either<String, &'static str>) -> usize {"] # [doc = "     either::for_both!(owned_or_borrowed, s => s.len())"] # [doc = " }"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     let borrowed = Either::Right(\"Hello world!\");"] # [doc = "     let owned = Either::Left(\"Hello world!\".to_owned());"] # [doc = ""] # [doc = "     assert_eq!(length(borrowed), 12);"] # [doc = "     assert_eq!(length(owned), 12);"] # [doc = " }"] # [doc = " ```"] # [macro_export] macro_rules ! for_both { ($ value : expr , $ pattern : pat => $ result : expr) => { match $ value { $ crate :: Either :: Left ($ pattern) => $ result , $ crate :: Either :: Right ($ pattern) => $ result , } } ; }
};
}
