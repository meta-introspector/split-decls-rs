macro_rules! deps {
    () => {
        PutBackN!();
        Itertools!();
    };
}

macro_rules! ExactlyOneError {
    () => {
        deps!();
        # [doc = " Iterator returned for the error case of `Itertools`"] # [doc = " [`exactly_one()`](crate::Itertools::exactly_one) and"] # [doc = " [`at_most_one()`](crate::Itertools::at_most_one)."] # [doc = " This iterator yields exactly the same elements as the input iterator."] # [doc = ""] # [doc = " During the execution of `exactly_one` the iterator must be mutated.  This wrapper"] # [doc = " effectively \"restores\" the state of the input iterator when it's handed back."] # [doc = ""] # [doc = " This is very similar to `PutBackN` except this iterator only supports 0-2 elements and does not"] # [doc = " use a `Vec`."] # [derive (Clone)] pub struct ExactlyOneError < I > where I : Iterator , { first_two : Option < Either < [I :: Item ; 2] , I :: Item > > , inner : I , }
    };
}

ExactlyOneError!();