macro_rules! deps {
    () => {
        StreamFuture!();
    };
}

macro_rules! IntoIter {
    () => {
        deps!();
        # [doc = " Owned iterator over all streams in the unordered set."] # [derive (Debug)] pub struct IntoIter < St : Unpin > (futures_unordered :: IntoIter < StreamFuture < St > >) ;
    };
}

IntoIter!()