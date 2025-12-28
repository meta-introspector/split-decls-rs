macro_rules! deps {
    () => {
        Ordering!();
        Store!();
        State!();
    };
}

macro_rules! AllObjects {
    () => {
        deps!();
        # [doc = " An iterator over all, _possibly duplicate_, objects of an object store, which by default uses no extra memory but yields an"] # [doc = " order that is costly to traverse when querying object information or decoding them."] # [doc = ""] # [doc = " Use [`with_ordering()`][AllObjects::with_ordering()] to choose a performance trade-off."] pub struct AllObjects { state : State , num_objects : usize , loose_dbs : Arc < Vec < loose :: Store > > , order : Ordering , }
    };
}

AllObjects!();