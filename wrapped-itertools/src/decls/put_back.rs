macro_rules! deps {
    () => {
        PutBack!();
    };
}

macro_rules! put_back {
    () => {
        deps!();
        # [doc = " Create an iterator where you can put back a single item"] pub fn put_back < I > (iterable : I) -> PutBack < I :: IntoIter > where I : IntoIterator , { PutBack { top : None , iter : iterable . into_iter () , } }
    };
}

put_back!();