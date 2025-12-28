macro_rules! type_parameter_of_option {
    () => {
        fn type_parameter_of_option (ty : & Type) -> Option < & Type > { let path = match ty { Type :: Path (ty) => & ty . path , _ => return None , } ; let last = path . segments . last () . unwrap () ; if last . ident != "Option" { return None ; } let bracketed = match & last . arguments { PathArguments :: AngleBracketed (bracketed) => bracketed , _ => return None , } ; if bracketed . args . len () != 1 { return None ; } match & bracketed . args [0] { GenericArgument :: Type (arg) => Some (arg) , _ => None , } }
    };
}

type_parameter_of_option!()