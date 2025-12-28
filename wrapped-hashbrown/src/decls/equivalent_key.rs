macro_rules! deps {
    () => {
        Equivalent!();
    };
}

macro_rules! equivalent_key {
    () => {
        deps!();
        # [doc = " Ensures that a single closure type across uses of this which, in turn prevents multiple"] # [doc = " instances of any functions like `RawTable::reserve` from being generated"] # [cfg_attr (feature = "inline-more" , inline)] pub (crate) fn equivalent_key < Q , K , V > (k : & Q) -> impl Fn (& (K , V)) -> bool + '_ where Q : Equivalent < K > + ? Sized , { move | x | k . equivalent (& x . 0) }
    };
}

equivalent_key!();