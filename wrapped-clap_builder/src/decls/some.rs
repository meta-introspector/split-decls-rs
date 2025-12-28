macro_rules! some {
    () => {
        macro_rules ! some { ($ expr : expr) => { match $ expr { Some (val) => val , None => { return None ; } } } ; }
    };
}

some!();