macro_rules! deps {
    () => {
        Equivalent!();
    };
}

macro_rules! equivalent {
    () => {
        deps!();
        # [doc = " Ensures that a single closure type across uses of this which, in turn prevents multiple"] # [doc = " instances of any functions like `RawTable::reserve` from being generated"] # [cfg_attr (feature = "inline-more" , inline)] # [allow (dead_code)] pub (crate) fn equivalent < Q , K > (k : & Q) -> impl Fn (& K) -> bool + '_ where Q : Equivalent < K > + ? Sized , { move | x | k . equivalent (x) }
    };
}

equivalent!()