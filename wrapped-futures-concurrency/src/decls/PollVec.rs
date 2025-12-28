macro_rules! deps {
    () => {
        PollState!();
    };
}

macro_rules! PollVec {
    () => {
        deps!();
        # [derive (Default)] pub (crate) struct PollVec (SmallVec < PollState , MAX_INLINE_ENTRIES >) ;
    };
}

PollVec!()