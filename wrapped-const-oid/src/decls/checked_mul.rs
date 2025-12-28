macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! checked_mul {
    () => {
        deps!();
        # [doc = " `const fn`-friendly checked multiplication helper."] macro_rules ! checked_mul { ($ a : expr , $ b : expr) => { match $ a . checked_mul ($ b) { Some (n) => n , None => return Err (Error :: Overflow) , } } ; }
    };
}

checked_mul!();