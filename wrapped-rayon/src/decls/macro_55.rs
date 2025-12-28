macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! macro_55 {
    () => {
        deps!();
        delegate_iterator ! { Iter <'a , T > => &'a T , impl <'a , T : Sync + 'a > }
    };
}

macro_55!()