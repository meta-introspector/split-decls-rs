macro_rules! deps {
    () => {
        PutBackN!();
    };
}

macro_rules! put_back_n {
    () => {
        deps!();
        # [doc = " Create an iterator where you can put back multiple values to the front"] # [doc = " of the iteration."] # [doc = ""] # [doc = " Iterator element type is `I::Item`."] pub fn put_back_n < I > (iterable : I) -> PutBackN < I :: IntoIter > where I : IntoIterator , { PutBackN { top : Vec :: new () , iter : iterable . into_iter () , } }
    };
}

put_back_n!();