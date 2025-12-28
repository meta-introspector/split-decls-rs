macro_rules! deps {
    () => {
        StateID!();
    };
}

macro_rules! Seen {
    () => {
        deps!();
        # [derive (Debug)] struct Seen { # [cfg (feature = "alloc")] set : alloc :: collections :: BTreeSet < StateID > , # [cfg (not (feature = "alloc"))] set : core :: marker :: PhantomData < StateID > , }
    };
}

Seen!();