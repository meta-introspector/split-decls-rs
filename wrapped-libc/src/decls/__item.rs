macro_rules! __item {
    () => {
        macro_rules ! __item { ($ i : item) => { $ i } ; }
    };
}

__item!()