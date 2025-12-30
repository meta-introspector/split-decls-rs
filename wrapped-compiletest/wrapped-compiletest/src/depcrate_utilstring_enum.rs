// Generated macro for string_enum (macro)
macro_rules! Depcrate_utilstring_enum {
() => {
// Module: crate::util
// Provides: {"string_enum"}
// Dependencies: {}
macro_rules ! string_enum { ($ (# [$ meta : meta]) * $ vis : vis enum $ name : ident { $ ($ variant : ident => $ repr : expr ,) * }) => { $ (# [$ meta]) * $ vis enum $ name { $ ($ variant ,) * } impl $ name { $ vis const VARIANTS : &'static [Self] = & [$ (Self ::$ variant ,) *] ; $ vis const STR_VARIANTS : &'static [&'static str] = & [$ (Self ::$ variant . to_str () ,) *] ; $ vis const fn to_str (& self) -> &'static str { match self { $ (Self ::$ variant => $ repr ,) * } } } impl :: std :: fmt :: Display for $ name { fn fmt (& self , f : & mut :: std :: fmt :: Formatter <'_ >) -> :: std :: fmt :: Result { :: std :: fmt :: Display :: fmt (self . to_str () , f) } } impl :: std :: str :: FromStr for $ name { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { $ ($ repr => Ok (Self ::$ variant) ,) * _ => Err (format ! (concat ! ("unknown `" , stringify ! ($ name) , "` variant: `{}`") , s)) , } } } } }
};
}
