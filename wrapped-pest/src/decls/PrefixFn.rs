macro_rules! deps {
    () => {
        Pair!();
    };
}

macro_rules! PrefixFn {
    () => {
        deps!();
        type PrefixFn < 'a , 'i , R , T > = Box < dyn FnMut (Pair < 'i , R > , T) -> T + 'a > ;
    };
}

PrefixFn!();