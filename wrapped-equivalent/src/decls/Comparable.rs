macro_rules! deps {
    () => {
        Equivalent!();
    };
}

macro_rules! Comparable {
    () => {
        deps!();
        # [doc = " Key ordering trait."] # [doc = ""] # [doc = " This trait allows ordered map lookup to be customized. It has one blanket"] # [doc = " implementation that uses the regular solution with `Borrow` and `Ord`, just"] # [doc = " like `BTreeMap` does, so that you can pass `&str` to lookup into a map with"] # [doc = " `String` keys and so on."] pub trait Comparable < K : ? Sized > : Equivalent < K > { # [doc = " Compare self to `key` and return their ordering."] fn compare (& self , key : & K) -> Ordering ; }
    };
}

Comparable!()