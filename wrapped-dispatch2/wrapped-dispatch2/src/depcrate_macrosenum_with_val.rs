// Generated macro for enum_with_val (macro)
macro_rules! Depcrate_macrosenum_with_val {
() => {
// Module: crate::macros
// Provides: {"enum_with_val"}
// Dependencies: {}
macro_rules ! enum_with_val { ($ (# [$ meta : meta]) * $ vis : vis struct $ ident : ident ($ innervis : vis $ ty : ty) { $ ($ (# [$ varmeta : meta]) * $ variant : ident = $ num : expr) ,* $ (,) * }) => { $ (# [$ meta]) * # [repr (transparent)] $ vis struct $ ident ($ innervis $ ty) ; # [allow (non_upper_case_globals)] impl $ ident { $ ($ (# [$ varmeta]) * $ vis const $ variant : Self = Self ($ num) ;) * } impl :: core :: fmt :: Debug for $ ident { fn fmt (& self , f : & mut :: core :: fmt :: Formatter <'_ >) -> :: core :: fmt :: Result { match self { $ (&$ ident ::$ variant => write ! (f , "{}::{}" , stringify ! ($ ident) , stringify ! ($ variant)) ,) * &$ ident (v) => write ! (f , "UNKNOWN({})" , v) , } } } # [cfg (feature = "objc2")] unsafe impl objc2 :: encode :: Encode for $ ident { const ENCODING : objc2 :: encode :: Encoding = <$ ty as objc2 :: encode :: Encode >:: ENCODING ; } # [cfg (feature = "objc2")] unsafe impl objc2 :: encode :: RefEncode for $ ident { const ENCODING_REF : objc2 :: encode :: Encoding = objc2 :: encode :: Encoding :: Pointer (&< Self as objc2 :: encode :: Encode >:: ENCODING) ; } } }
};
}
