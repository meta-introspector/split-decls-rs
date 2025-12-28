macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! checked_sub {
    () => {
        deps!();
        # [doc = " `const fn`-friendly checked subtraction helper."] macro_rules ! checked_sub { ($ a : expr , $ b : expr) => { match $ a . checked_sub ($ b) { Some (n) => n , None => return Err (Error :: Overflow) , } } ; }
    };
}

checked_sub!()