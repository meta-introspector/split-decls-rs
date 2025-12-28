macro_rules! Iterate {
    () => {
        # [doc = " An iterator that infinitely applies function to value and yields results."] # [doc = ""] # [doc = " This `struct` is created by the [`iterate()`](crate::iterate) function."] # [doc = " See its documentation for more."] # [derive (Clone)] # [must_use = "iterators are lazy and do nothing unless consumed"] pub struct Iterate < St , F > { state : St , f : F , }
    };
}

Iterate!()