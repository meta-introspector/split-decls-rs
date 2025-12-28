macro_rules! macro_1 {
    () => {
        # [cfg (doctests)] # [cfg (doctest)] doctest ! ("../README.md") ;
    };
}

macro_1!();