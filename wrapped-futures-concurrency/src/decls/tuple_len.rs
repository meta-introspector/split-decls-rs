macro_rules! tuple_len {
    () => {
        # [doc = " Calculate the number of tuples currently being operated on."] macro_rules ! tuple_len { (@ count_one $ F : ident) => (1) ; ($ ($ F : ident ,) *) => (0 $ (+ crate :: utils :: tuple_len ! (@ count_one $ F)) *) ; }
    };
}

tuple_len!()