macro_rules! FALSE_LITERALS {
    () => {
        # [doc = " False values are `n`, `no`, `f`, `false`, `off`, and `0`."] pub (crate) const FALSE_LITERALS : [& str ; 6] = ["n" , "no" , "f" , "false" , "off" , "0"] ;
    };
}

FALSE_LITERALS!();