macro_rules! deps {
    () => {
        FromParallelIterator!();
        IntoParallelIterator!();
    };
}

macro_rules! collect_string {
    () => {
        deps!();
        macro_rules ! collect_string { ($ desc : literal , $ item : ty $ (, $ a : lifetime) ?) => { # [doc = concat ! ("Collects " , $ desc , " from a parallel iterator into a string.")] impl $ (<$ a >) ? FromParallelIterator <$ item > for String { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = $ item >, { collect_extended (par_iter) } } # [doc = concat ! ("Collects " , $ desc , " from a parallel iterator into a boxed string.")] impl $ (<$ a >) ? FromParallelIterator <$ item > for Box < str > { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = $ item >, { String :: from_par_iter (par_iter) . into_boxed_str () } } } }
    };
}

collect_string!()