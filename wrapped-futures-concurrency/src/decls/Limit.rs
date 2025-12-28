macro_rules! deps {
    () => {
        ConcurrentStream!();
    };
}

macro_rules! Limit {
    () => {
        deps!();
        # [doc = " A concurrent iterator that limits the amount of concurrency applied."] # [doc = ""] # [doc = " This `struct` is created by the [`limit`] method on [`ConcurrentStream`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`limit`]: ConcurrentStream::limit"] # [doc = " [`ConcurrentStream`]: trait.ConcurrentStream.html"] # [derive (Debug)] pub struct Limit < CS : ConcurrentStream > { inner : CS , limit : Option < NonZeroUsize > , }
    };
}

Limit!()