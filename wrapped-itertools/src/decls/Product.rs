macro_rules! Product {
    () => {
        # [derive (Debug , Clone)] # [doc = " An iterator adaptor that iterates over the cartesian product of"] # [doc = " the element sets of two iterators `I` and `J`."] # [doc = ""] # [doc = " Iterator element type is `(I::Item, J::Item)`."] # [doc = ""] # [doc = " See [`.cartesian_product()`](crate::Itertools::cartesian_product) for more information."] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct Product < I , J > where I : Iterator , { a : I , # [doc = " `a_cur` is `None` while no item have been taken out of `a` (at definition)."] # [doc = " Then `a_cur` will be `Some(Some(item))` until `a` is exhausted,"] # [doc = " in which case `a_cur` will be `Some(None)`."] a_cur : Option < Option < I :: Item > > , b : J , b_orig : J , }
    };
}

Product!();