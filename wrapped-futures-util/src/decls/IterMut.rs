macro_rules! deps {
    () => {
        StreamFuture!();
    };
}

macro_rules! IterMut {
    () => {
        deps!();
        # [doc = " Mutable iterator over all streams in the unordered set."] # [derive (Debug)] pub struct IterMut < 'a , St : Unpin > (futures_unordered :: IterMut < 'a , StreamFuture < St > >) ;
    };
}

IterMut!()