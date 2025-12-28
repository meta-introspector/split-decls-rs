macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! r#try_recurse {
    () => {
        deps!();
        macro_rules ! r#try_recurse { ($ expr : expr $ (,) ?) => { match $ expr { Result :: Err (error :: Error :: TooMuchRecursion) => { return Result :: Err (error :: Error :: TooMuchRecursion) ; } val => val , } } ; }
    };
}

r#try_recurse!()