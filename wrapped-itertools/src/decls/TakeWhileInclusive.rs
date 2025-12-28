macro_rules! TakeWhileInclusive {
    () => {
        # [doc = " An iterator adaptor that consumes elements while the given predicate is"] # [doc = " `true`, including the element for which the predicate first returned"] # [doc = " `false`."] # [doc = ""] # [doc = " See [`.take_while_inclusive()`](crate::Itertools::take_while_inclusive)"] # [doc = " for more information."] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone)] pub struct TakeWhileInclusive < I , F > { iter : I , predicate : F , done : bool , }
    };
}

TakeWhileInclusive!()