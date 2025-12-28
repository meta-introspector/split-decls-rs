macro_rules! deps {
    () => {
        LookSet!();
    };
}

macro_rules! LookSetIter {
    () => {
        deps!();
        # [doc = " An iterator over all look-around assertions in a [`LookSet`]."] # [doc = ""] # [doc = " This iterator is created by [`LookSet::iter`]."] # [derive (Clone , Debug)] pub struct LookSetIter { set : LookSet , }
    };
}

LookSetIter!()