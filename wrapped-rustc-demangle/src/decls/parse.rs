macro_rules! parse {
    () => {
        # [doc = " Call a parser method (if the parser hasn't errored yet),"] # [doc = " and mark the parser as errored if it returns `Err`."] # [doc = ""] # [doc = " If the parser errored, before or now, this returns early,"] # [doc = " from the current function, after printing either:"] # [doc = " * for a new error, the appropriate message (see `ParseError::message`)"] # [doc = " * for an earlier error, only `?` -  this allows callers to keep printing"] # [doc = "   the approximate syntax of the path/type/const, despite having errors,"] # [doc = "   e.g. `Vec<[(A, ?); ?]>` instead of `Vec<[(A, ?`"] macro_rules ! parse { ($ printer : ident , $ method : ident $ (($ ($ arg : expr) ,*)) *) => { match $ printer . parser { Ok (ref mut parser) => match parser .$ method ($ ($ ($ arg) ,*) *) { Ok (x) => x , Err (err) => { $ printer . print (err . message ()) ?; $ printer . parser = Err (err) ; return Ok (()) ; } } Err (_) => return $ printer . print ("?") , } } ; }
    };
}

parse!();