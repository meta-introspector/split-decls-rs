macro_rules! deps {
    () => {
        ConcurrentStream!();
    };
}

macro_rules! Take {
    () => {
        deps!();
        # [doc = " A concurrent iterator that only iterates over the first `n` iterations of `iter`."] # [doc = ""] # [doc = " This `struct` is created by the [`take`] method on [`ConcurrentStream`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`take`]: ConcurrentStream::take"] # [doc = " [`ConcurrentStream`]: trait.ConcurrentStream.html"] # [derive (Debug)] pub struct Take < CS : ConcurrentStream > { inner : CS , limit : usize , }
    };
}

Take!();