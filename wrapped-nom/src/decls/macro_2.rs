macro_rules! macro_2 {
    () => {
        # [cfg (doctest)] doc_comment :: doctest ! ("../README.md") ;
    };
}

macro_2!();