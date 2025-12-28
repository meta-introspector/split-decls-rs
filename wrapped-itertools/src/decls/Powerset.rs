macro_rules! deps {
    () => {
        Combinations!();
    };
}

macro_rules! Powerset {
    () => {
        deps!();
        # [doc = " An iterator to iterate through the powerset of the elements from an iterator."] # [doc = ""] # [doc = " See [`.powerset()`](crate::Itertools::powerset) for more"] # [doc = " information."] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct Powerset < I : Iterator > { combs : Combinations < I > , }
    };
}

Powerset!();