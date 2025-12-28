macro_rules! TakeWhile {
    () => {
        # [doc = " An iterator which only returns initial elements matching a predicate."] pub struct TakeWhile < I , F > { it : I , f : F , done : bool , }
    };
}

TakeWhile!()