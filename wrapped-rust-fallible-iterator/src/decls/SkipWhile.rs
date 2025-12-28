macro_rules! SkipWhile {
    () => {
        # [doc = " An iterator which skips initial elements based on a predicate."] # [derive (Clone , Debug)] pub struct SkipWhile < I , P > { it : I , flag : bool , predicate : P , }
    };
}

SkipWhile!()