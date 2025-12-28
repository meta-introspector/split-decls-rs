macro_rules! macro_5 {
    () => {
        litemap_impl ! (not (feature = "alloc") , S) ;
    };
}

macro_5!();