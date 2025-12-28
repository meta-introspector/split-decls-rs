macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! macro_44 {
    () => {
        deps!();
        delegate_iterator ! { Iter <'a , K , V > => (&'a K , &'a V) , impl <'a , K : Sync + 'a , V : Sync + 'a > }
    };
}

macro_44!();