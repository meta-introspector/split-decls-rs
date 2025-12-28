macro_rules! deps {
    () => {
        Version!();
    };
}

macro_rules! version_cmp {
    () => {
        deps!();
        # [test] fn version_cmp () { use super :: version :: Version ; let v123 = Version :: new (1 , 2 , 3) ; assert ! (Version :: new (1 , 0 , 0) < v123) ; assert ! (Version :: new (1 , 2 , 2) < v123) ; assert ! (Version :: new (1 , 2 , 3) == v123) ; assert ! (Version :: new (1 , 2 , 4) > v123) ; assert ! (Version :: new (1 , 10 , 0) > v123) ; assert ! (Version :: new (2 , 0 , 0) > v123) ; }
    };
}

version_cmp!()