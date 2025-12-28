macro_rules! type_is_backtrace {
    () => {
        fn type_is_backtrace (ty : & Type) -> bool { let path = match ty { Type :: Path (ty) => & ty . path , _ => return false , } ; let last = path . segments . last () . unwrap () ; last . ident == "Backtrace" && last . arguments . is_empty () }
    };
}

type_is_backtrace!()