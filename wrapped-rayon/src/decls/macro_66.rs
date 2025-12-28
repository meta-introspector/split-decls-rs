macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! macro_66 {
    () => {
        deps!();
        delegate_iterator ! { IterMut <'a , K , V > => (&'a K , &'a mut V) , impl <'a , K : Sync , V : Send > }
    };
}

macro_66!()