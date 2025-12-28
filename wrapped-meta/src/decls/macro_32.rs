macro_rules! deps {
    () => {
        Rule!();
        Expr!();
    };
}

macro_rules! macro_32 {
    () => {
        deps!();
        to_hash_map ! (to_hash_map , Rule , Expr) ;
    };
}

macro_32!();