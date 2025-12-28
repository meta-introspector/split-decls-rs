macro_rules! IteratorIndex {
    () => {
        # [doc = " Used by [`Itertools::get`] to know which iterator"] # [doc = " to turn different ranges into."] pub trait IteratorIndex < I > : private_iter_index :: Sealed where I : Iterator , { # [doc = " The type returned for this type of index."] type Output : Iterator < Item = I :: Item > ; # [doc = " Returns an adapted iterator for the current index."] # [doc = ""] # [doc = " Prefer calling [`Itertools::get`] instead"] # [doc = " of calling this directly."] fn index (self , from : I) -> Self :: Output ; }
    };
}

IteratorIndex!();