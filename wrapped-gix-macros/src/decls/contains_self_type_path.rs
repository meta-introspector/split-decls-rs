macro_rules! contains_self_type_path {
    () => {
        fn contains_self_type_path (path : & Path) -> bool { path . segments . iter () . any (| segment | { segment . ident == "Self" || match & segment . arguments { PathArguments :: AngleBracketed (AngleBracketedGenericArguments { args , .. }) => { args . iter () . any (| generic_arg | match generic_arg { GenericArgument :: Type (ty) => contains_self_type (ty) , GenericArgument :: Const (expr) => contains_self_type_expr (expr) , _ => false , }) } PathArguments :: Parenthesized (ParenthesizedGenericArguments { inputs , output , .. }) => { inputs . iter () . any (contains_self_type) || matches ! (output , ReturnType :: Type (_ , ty) if contains_self_type (ty)) } _ => false , } }) }
    };
}

contains_self_type_path!();