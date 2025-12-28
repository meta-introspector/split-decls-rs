macro_rules! contains_non_static_lifetime {
    () => {
        fn contains_non_static_lifetime (ty : & Type) -> bool { match ty { Type :: Path (ty) => { let bracketed = match & ty . path . segments . last () . unwrap () . arguments { PathArguments :: AngleBracketed (bracketed) => bracketed , _ => return false , } ; for arg in & bracketed . args { match arg { GenericArgument :: Type (ty) if contains_non_static_lifetime (ty) => return true , GenericArgument :: Lifetime (lifetime) if lifetime . ident != "static" => { return true } _ => { } } } false } Type :: Reference (ty) => ty . lifetime . as_ref () . map_or (false , | lifetime | lifetime . ident != "static") , _ => false , } }
    };
}

contains_non_static_lifetime!();