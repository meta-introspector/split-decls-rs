macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! checked_add {
    () => {
        deps!();
        # [doc = " `const fn`-friendly checked addition helper."] macro_rules ! checked_add { ($ a : expr , $ b : expr) => { match $ a . checked_add ($ b) { Some (n) => n , None => return Err (Error :: Overflow) , } } ; }
    };
}

checked_add!()