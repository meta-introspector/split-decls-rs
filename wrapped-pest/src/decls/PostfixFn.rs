macro_rules! deps {
    () => {
        Pair!();
    };
}

macro_rules! PostfixFn {
    () => {
        deps!();
        type PostfixFn < 'a , 'i , R , T > = Box < dyn FnMut (T , Pair < 'i , R >) -> T + 'a > ;
    };
}

PostfixFn!();