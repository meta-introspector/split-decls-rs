macro_rules! SkipWhile {
    () => {
        # [doc = " An iterator which skips initial elements matching a predicate."] pub struct SkipWhile < I , F > { it : I , f : F , done : bool , }
    };
}

SkipWhile!()