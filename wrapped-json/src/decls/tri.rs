macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! tri {
    () => {
        deps!();
        macro_rules ! tri { ($ e : expr $ (,) ?) => { match $ e { core :: result :: Result :: Ok (val) => val , core :: result :: Result :: Err (err) => return core :: result :: Result :: Err (err) , } } ; }
    };
}

tri!()