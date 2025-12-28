macro_rules! not_zero_impls {
    () => {
        macro_rules ! not_zero_impls { ($ ($ t : ty) ,*) => { $ (not_zero_impl ! ($ t , 0) ;) * } }
    };
}

not_zero_impls!()