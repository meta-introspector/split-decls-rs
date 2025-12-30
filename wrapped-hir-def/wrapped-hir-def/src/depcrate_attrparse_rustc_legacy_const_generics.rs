// Generated macro for parse_rustc_legacy_const_generics (function)
macro_rules! Depcrate_attrparse_rustc_legacy_const_generics {
() => {
// Module: crate::attr
// Provides: {"parse_rustc_legacy_const_generics"}
// Dependencies: {}
fn parse_rustc_legacy_const_generics (tt : & crate :: tt :: TopSubtree) -> Box < [u32] > { let mut indices = Vec :: new () ; let mut iter = tt . iter () ; while let (Some (first) , second) = (iter . next () , iter . next ()) { match first { TtElement :: Leaf (tt :: Leaf :: Literal (lit)) => match lit . symbol . as_str () . parse () { Ok (index) => indices . push (index) , Err (_) => break , } , _ => break , } if let Some (comma) = second { match comma { TtElement :: Leaf (tt :: Leaf :: Punct (punct)) if punct . char == ',' => { } _ => break , } } } indices . into_boxed_slice () }
};
}
