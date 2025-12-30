// Generated macro for iterate (function)
macro_rules! Depcrate_parser_repeatiterate {
() => {
// Module: crate::parser::repeat
// Provides: {"iterate"}
// Dependencies: {}
# [doc = ""] # [doc = " ```"] # [doc = " # use combine::parser::repeat::{count_min_max, iterate};"] # [doc = " # use combine::*;"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     iterate(0..3, |&i, _| count_min_max(i, i, any())).parse(\"abbccc\"),"] # [doc = "     Ok((vec![\"\".to_string(), \"a\".to_string(), \"bb\".to_string()], \"ccc\")),"] # [doc = " );"] # [doc = " ```"] pub fn iterate < F , J , P , I , Q > (iterable : J , parser : P) -> Iterate < F , J , P > where P : FnMut (& J :: Item , & mut I) -> Q , Q : Parser < I > , I : Stream , J : IntoIterator + Clone , F : Extend < Q :: Output > + Default , { Iterate { parser , iterable , _marker : PhantomData , } }
};
}
