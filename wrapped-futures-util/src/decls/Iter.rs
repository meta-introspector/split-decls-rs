macro_rules! deps {
    () => {
        StreamFuture!();
    };
}

macro_rules! Iter {
    () => {
        deps!();
        # [doc = " Immutable iterator over all streams in the unordered set."] # [derive (Debug)] pub struct Iter < 'a , St : Unpin > (futures_unordered :: Iter < 'a , StreamFuture < St > >) ;
    };
}

Iter!();