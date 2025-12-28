macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! macro_63 {
    () => {
        deps!();
        delegate_iterator ! { Iter <'a , K , V > => (&'a K , &'a V) , impl <'a , K : Sync , V : Sync > }
    };
}

macro_63!();