// Generated macro for impl_arbitrary (module)
macro_rules! Depcrateimpl_arbitrary {
() => {
// Module: crate
// Provides: {"impl_arbitrary"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] mod impl_arbitrary { use super :: { FloatIsNan , NotNan , OrderedFloat } ; use arbitrary :: { Arbitrary , Unstructured } ; use num_traits :: FromPrimitive ; macro_rules ! impl_arbitrary { ($ ($ f : ident) ,+) => { $ (impl <'a > Arbitrary <'a > for NotNan <$ f > { fn arbitrary (u : & mut Unstructured <'a >) -> arbitrary :: Result < Self > { let float : $ f = u . arbitrary () ?; match NotNan :: new (float) { Ok (notnan_value) => Ok (notnan_value) , Err (FloatIsNan) => { let (mantissa , _exponent , sign) = num_traits :: float :: FloatCore :: integer_decode (float) ; let revised_float = <$ f >:: from_i64 (i64 :: from (sign) * mantissa as i64) . unwrap () ; Ok (NotNan :: new (revised_float) . unwrap ()) } } } fn size_hint (depth : usize) -> (usize , Option < usize >) { <$ f as Arbitrary >:: size_hint (depth) } } impl <'a > Arbitrary <'a > for OrderedFloat <$ f > { fn arbitrary (u : & mut Unstructured <'a >) -> arbitrary :: Result < Self > { let float : $ f = u . arbitrary () ?; Ok (OrderedFloat :: from (float)) } fn size_hint (depth : usize) -> (usize , Option < usize >) { <$ f as Arbitrary >:: size_hint (depth) } }) * } } impl_arbitrary ! { f32 , f64 } }
};
}
