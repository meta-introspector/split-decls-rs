// Generated macro for simple_newtypes (macro)
macro_rules! Depcratesimple_newtypes {
() => {
// Module: crate
// Provides: {"simple_newtypes"}
// Dependencies: {}
macro_rules ! simple_newtypes { ($ ($ (# [$ attr : meta]) * type $ name : ident = $ oldty : ty where default = $ default : expr , display = $ format : expr ;) *) => { $ ($ (# [$ attr]) * # [derive (Clone , Copy , Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct $ name (pub $ oldty) ; impl Default for $ name { # [inline] fn default () -> Self { $ name ($ default) } } impl From <$ oldty > for $ name { fn from (x : $ oldty) -> $ name { $ name (x) } } impl From <$ name > for $ oldty { fn from ($ name (x) : $ name) -> $ oldty { x } } impl fmt :: Display for $ name { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , $ format , self . 0) } }) * } }
};
}
