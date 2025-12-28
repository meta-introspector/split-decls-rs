macro_rules! fun_to_string {
    () => {
        fn fun_to_string (decl : & ast :: FnDecl , header : ast :: FnHeader , ident : Ident , generics : & ast :: Generics ,) -> String { to_string (| s | { let (cb , ib) = s . head ("") ; s . print_fn (decl , header , Some (ident) , generics) ; s . end (ib) ; s . end (cb) ; }) }
    };
}

fun_to_string!();