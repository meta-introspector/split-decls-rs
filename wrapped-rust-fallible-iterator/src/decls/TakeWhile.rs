macro_rules! TakeWhile {
    () => {
        # [doc = " An iterator which yields elements based on a predicate."] # [derive (Clone , Debug)] pub struct TakeWhile < I , P > { it : I , flag : bool , predicate : P , }
    };
}

TakeWhile!()