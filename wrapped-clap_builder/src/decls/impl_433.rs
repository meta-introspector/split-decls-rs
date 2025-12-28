macro_rules! deps {
    () => {
        Result!();
        MatchesError!();
    };
}

macro_rules! impl_433 {
    () => {
        deps!();
        impl MatchesError { # [cfg_attr (debug_assertions , track_caller)] pub (crate) fn unwrap < T > (id : & str , r : Result < T , MatchesError >) -> T { let err = match r { Ok (t) => { return t ; } Err (err) => err , } ; panic ! ("Mismatch between definition and access of `{id}`. {err}" ,) } }
    };
}

impl_433!();