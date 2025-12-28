macro_rules! IntoFallible {
    () => {
        # [doc = " A fallible iterator that wraps a normal iterator over `Result`s."] # [derive (Clone , Debug)] pub struct IntoFallible < I > (I) ;
    };
}

IntoFallible!();