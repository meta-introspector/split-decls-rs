macro_rules! deps {
    () => {
        ConcurrentStream!();
    };
}

macro_rules! IntoConcurrentStream {
    () => {
        deps!();
        # [doc = " Conversion into a [`ConcurrentStream`]"] pub trait IntoConcurrentStream { # [doc = " The type of the elements being iterated over."] type Item ; # [doc = " Which kind of iterator are we turning this into?"] type IntoConcurrentStream : ConcurrentStream < Item = Self :: Item > ; # [doc = " Convert `self` into a concurrent iterator."] fn into_co_stream (self) -> Self :: IntoConcurrentStream ; }
    };
}

IntoConcurrentStream!();