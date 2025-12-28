macro_rules! macro_1 {
    () => {
        # [cfg (doctest)] doctest ! ("../README.md") ;
    };
}

macro_1!();