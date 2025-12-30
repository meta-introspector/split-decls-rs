// Generated macro for implement (macro)
macro_rules! Depcrateimplement {
() => {
// Module: crate
// Provides: {"implement"}
// Dependencies: {}
# [doc = " Macro for implementing an opaque `Debug` implementation."] # [macro_export] macro_rules ! implement { ($ struct : ident <$ ($ params : ident) ,+>) => { impl <$ ($ params) ,+> $ crate :: __core :: fmt :: Debug for $ struct <$ ($ params) ,+> { fn fmt (& self , f : & mut $ crate :: __core :: fmt :: Formatter ,) -> Result < () , $ crate :: __core :: fmt :: Error > { write ! (f , concat ! (stringify ! ($ struct) , "<" , $ crate :: format_params ! ($ ($ params) ,+) , "> {{ ... }}") , $ ($ crate :: __core :: any :: type_name ::<$ params > ()) ,+) } } } ; ($ struct : ty) => { impl $ crate :: __core :: fmt :: Debug for $ struct { fn fmt (& self , f : & mut $ crate :: __core :: fmt :: Formatter ,) -> Result < () , $ crate :: __core :: fmt :: Error > { write ! (f , concat ! (stringify ! ($ struct) , " {{ ... }}")) } } } ; }
};
}
