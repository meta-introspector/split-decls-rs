// Generated macro for transpose_array (macro)
macro_rules! Depcrate_base_constructiontranspose_array {
() => {
// Module: crate::base::construction
// Provides: {"transpose_array"}
// Dependencies: {}
macro_rules ! transpose_array ([$ ($ a : ident) ,*;] => { [$ ([$ a]) ,*] } ; [$ ($ a : ident) ,*; $ ($ b : ident) ,*;] => { [$ ([$ a , $ b]) ,*] } ; [$ ($ a : ident) ,*; $ ($ b : ident) ,*; $ ($ c : ident) ,*;] => { [$ ([$ a , $ b , $ c]) ,*] } ; [$ ($ a : ident) ,*; $ ($ b : ident) ,*; $ ($ c : ident) ,*; $ ($ d : ident) ,*;] => { [$ ([$ a , $ b , $ c , $ d]) ,*] } ; [$ ($ a : ident) ,*; $ ($ b : ident) ,*; $ ($ c : ident) ,*; $ ($ d : ident) ,*; $ ($ e : ident) ,*;] => { [$ ([$ a , $ b , $ c , $ d , $ e]) ,*] } ; [$ ($ a : ident) ,*; $ ($ b : ident) ,*; $ ($ c : ident) ,*; $ ($ d : ident) ,*; $ ($ e : ident) ,*; $ ($ f : ident) ,*;] => { [$ ([$ a , $ b , $ c , $ d , $ e , $ f]) ,*] } ;) ;
};
}
