macro_rules! BASE_FN {
    () => {
        # [doc = " DepNodes for functions + methods"] const BASE_FN : & [& str] = & [label_strs :: fn_sig , label_strs :: generics_of , label_strs :: predicates_of , label_strs :: type_of , label_strs :: typeck ,] ;
    };
}

BASE_FN!();