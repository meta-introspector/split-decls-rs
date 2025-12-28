macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! unpoison {
    () => {
        deps!();
        # [inline] fn unpoison < T > (r : Result < T , std :: sync :: PoisonError < T > >) -> T { r . unwrap_or_else (std :: sync :: PoisonError :: into_inner) }
    };
}

unpoison!()