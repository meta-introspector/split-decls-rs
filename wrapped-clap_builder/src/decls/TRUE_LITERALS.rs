macro_rules! TRUE_LITERALS {
    () => {
        # [doc = " True values are `y`, `yes`, `t`, `true`, `on`, and `1`."] pub (crate) const TRUE_LITERALS : [& str ; 6] = ["y" , "yes" , "t" , "true" , "on" , "1"] ;
    };
}

TRUE_LITERALS!();