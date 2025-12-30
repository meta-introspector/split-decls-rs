// Generated macro for named_arguments_tuple (macro)
macro_rules! Depcrate_sugarnamed_arguments_tuple {
() => {
// Module: crate::sugar
// Provides: {"named_arguments_tuple"}
// Dependencies: {}
macro_rules ! named_arguments_tuple { ($ ($ ix : tt $ argn : ident $ argv : ident) *) => { impl <'a , $ ($ argn : Copy) ,*, $ ($ argv) ,*> fmt :: Debug for NamedArguments < ($ ($ argn ,) *) ,&'a ($ ($ argv ,) *) > where $ (NamedArguments <$ argn , &'a $ argv > : fmt :: Debug) ,*, $ ($ argv : 'a) ,* { # [allow (unused_assignments)] fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut first = true ; $ (if ! first { write ! (f , ", ") ?; } first = false ; fmt :: Debug :: fmt (& NamedArguments ((self . 0) .$ ix , & (self . 1) .$ ix) , f) ?;) * Ok (()) } } impl <$ ($ argn : Copy) ,*, $ ($ argv) ,*> fmt :: Debug for NamedArguments < ($ ($ argn ,) *) , ($ ($ argv ,) *) > where $ (for <'a > NamedArguments <$ argn , &'a $ argv > : fmt :: Debug) ,* { # [allow (unused_assignments)] fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut first = true ; $ (if ! first { write ! (f , ", ") ?; } first = false ; fmt :: Debug :: fmt (& NamedArguments ((self . 0) .$ ix , & (self . 1) .$ ix) , f) ?;) * Ok (()) } } } }
};
}
