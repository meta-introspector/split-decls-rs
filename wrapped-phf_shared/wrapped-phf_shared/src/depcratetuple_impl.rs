// Generated macro for tuple_impl (macro)
macro_rules! Depcratetuple_impl {
() => {
// Module: crate
// Provides: {"tuple_impl"}
// Dependencies: {}
macro_rules ! tuple_impl { ($ ($ t : ident) ,+) => { impl <$ ($ t : PhfHash) ,+> PhfHash for ($ ($ t ,) +) { fn phf_hash < HS : Hasher > (& self , state : & mut HS) { # [allow (non_snake_case)] let ($ ($ t ,) +) = self ; $ ($ t . phf_hash (state) ;) + } } impl <$ ($ t : PhfHash) ,+> PhfBorrow < ($ ($ t ,) +) > for ($ ($ t ,) +) { fn borrow (& self) -> & ($ ($ t ,) +) { self } } impl <$ ($ t : FmtConst) ,+> FmtConst for ($ ($ t ,) +) { fn fmt_const (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { # [allow (non_snake_case)] let ($ ($ t ,) +) = self ; write ! (f , "(") ?; let mut first = true ; $ (if ! core :: mem :: replace (& mut first , false) { write ! (f , ", ") ?; } $ t . fmt_const (f) ?;) + write ! (f , ")") } } } ; }
};
}
