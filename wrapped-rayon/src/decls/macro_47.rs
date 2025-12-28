macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! macro_47 {
    () => {
        deps!();
        delegate_iterator ! { IterMut <'a , K , V > => (&'a K , &'a mut V) , impl <'a , K : Sync + 'a , V : Send + 'a > }
    };
}

macro_47!();