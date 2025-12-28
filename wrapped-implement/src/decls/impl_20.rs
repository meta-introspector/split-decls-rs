macro_rules! deps {
    () => {
        UseTree2!();
        ImplementType!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl UseTree2 { fn to_element_type (& self , namespace : & mut String) -> syn :: parse :: Result < ImplementType > { match self { Self :: Path (input) => { if ! namespace . is_empty () { namespace . push_str ("::") ; } namespace . push_str (& input . ident . to_string ()) ; input . tree . to_element_type (namespace) } Self :: Name (input) => { let mut type_name = input . ident . to_string () ; let span = input . ident . span () ; if ! namespace . is_empty () { type_name = format ! ("{namespace}::{type_name}") ; } let mut generics = vec ! [] ; for g in & input . generics { generics . push (g . to_element_type (& mut String :: new ()) ?) ; } Ok (ImplementType { type_name , generics , span , }) } Self :: Group (input) => Err (syn :: parse :: Error :: new (input . brace_token . span . join () , "Syntax not supported" ,)) , _ => unimplemented ! () , } } }
    };
}

impl_20!()