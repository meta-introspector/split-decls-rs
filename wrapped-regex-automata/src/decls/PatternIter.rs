macro_rules! deps {
    () => {
        NFA!();
    };
}

macro_rules! PatternIter {
    () => {
        deps!();
        # [doc = " An iterator over all pattern IDs in an NFA."] # [doc = ""] # [doc = " This iterator is created by [`NFA::patterns`]."] # [doc = ""] # [doc = " The lifetime parameter `'a` refers to the lifetime of the NFA from which"] # [doc = " this pattern iterator was created."] # [derive (Debug)] pub struct PatternIter < 'a > { it : PatternIDIter , # [doc = " We explicitly associate a lifetime with this iterator even though we"] # [doc = " don't actually borrow anything from the NFA. We do this for backward"] # [doc = " compatibility purposes. If we ever do need to borrow something from"] # [doc = " the NFA, then we can and just get rid of this marker without breaking"] # [doc = " the public API."] _marker : core :: marker :: PhantomData < & 'a () > , }
    };
}

PatternIter!()