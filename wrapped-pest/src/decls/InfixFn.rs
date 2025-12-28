macro_rules! deps {
    () => {
        Pair!();
    };
}

macro_rules! InfixFn {
    () => {
        deps!();
        type InfixFn < 'a , 'i , R , T > = Box < dyn FnMut (T , Pair < 'i , R > , T) -> T + 'a > ;
    };
}

InfixFn!();