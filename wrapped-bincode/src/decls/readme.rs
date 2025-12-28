macro_rules! readme {
    () => {
        # [cfg (all (feature = "alloc" , feature = "derive" , doctest))] mod readme { # ! [doc = include_str ! ("../readme.md")] }
    };
}

readme!()