// Generated macro for deselfify (function)
macro_rules! Depcratedeselfify {
() => {
// Module: crate
// Provides: {"deselfify"}
// Dependencies: {}
# [doc = " Replace any references to `Self` in `literal_type` with `actual`."] # [doc = " `generics` is the Generics field of the parent struct.  Useful for"] # [doc = " constructor methods."] fn deselfify (literal_type : & mut Type , actual : & Ident , generics : & Generics) { match literal_type { Type :: Slice (s) => { deselfify (s . elem . as_mut () , actual , generics) ; } , Type :: Array (a) => { deselfify (a . elem . as_mut () , actual , generics) ; } , Type :: Ptr (p) => { deselfify (p . elem . as_mut () , actual , generics) ; } , Type :: Reference (r) => { deselfify (r . elem . as_mut () , actual , generics) ; } , Type :: Tuple (tuple) => { for elem in tuple . elems . iter_mut () { deselfify (elem , actual , generics) ; } } Type :: Path (type_path) => { if let Some (ref mut qself) = type_path . qself { deselfify (qself . ty . as_mut () , actual , generics) ; } deselfify_path (& mut type_path . path , actual , generics) ; } , Type :: Paren (p) => { deselfify (p . elem . as_mut () , actual , generics) ; } , Type :: Group (g) => { deselfify (g . elem . as_mut () , actual , generics) ; } , Type :: Macro (_) | Type :: Verbatim (_) => { compile_error (literal_type . span () , "mockall_derive does not support this type as a return argument") ; } , Type :: TraitObject (tto) => { for bound in tto . bounds . iter_mut () { if let TypeParamBound :: Trait (t) = bound { deselfify_path (& mut t . path , actual , generics) ; } } } , Type :: ImplTrait (_) => { } , Type :: BareFn (_) => { } , Type :: Infer (_) | Type :: Never (_) => { } , _ => compile_error (literal_type . span () , "Unsupported type") , } }
};
}
