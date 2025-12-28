macro_rules! IntersperseFolder {
    () => {
        struct IntersperseFolder < C , T > { base : C , item : T , clone_first : bool , }
    };
}

IntersperseFolder!();