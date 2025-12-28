macro_rules! deps {
    () => {
        OptimizedExpr!();
    };
}

macro_rules! child_modifies_state {
    () => {
        deps!();
        fn child_modifies_state (expr : & OptimizedExpr , rules : & HashMap < String , OptimizedExpr > , cache : & mut HashMap < String , Option < bool > > ,) -> bool { expr . iter_top_down () . any (| expr | match expr { OptimizedExpr :: Push (_) => true , OptimizedExpr :: Ident (ref name) if name == "DROP" => true , OptimizedExpr :: Ident (ref name) if name == "POP" => true , OptimizedExpr :: Ident (ref name) => match cache . get (name) . cloned () { Some (option) => match option { Some (cached) => cached , None => { cache . insert (name . to_owned () , Some (false)) ; false } } , None => { cache . insert (name . to_owned () , None) ; let result = match rules . get (name) { Some (expr) => child_modifies_state (expr , rules , cache) , None => false , } ; cache . insert (name . to_owned () , Some (result)) ; result } } , _ => false , }) }
    };
}

child_modifies_state!();