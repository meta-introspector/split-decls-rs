macro_rules! MIN_SPACE {
    () => {
        # [doc = " Every line is allowed at least this much space, even if highly indented."] const MIN_SPACE : isize = 60 ;
    };
}

MIN_SPACE!();