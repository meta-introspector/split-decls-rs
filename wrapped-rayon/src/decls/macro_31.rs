macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! macro_31 {
    () => {
        deps!();
        delegate_indexed_iterator ! { Iter <'a , T > => &'a T , impl <'a , T : Sync + 'a > }
    };
}

macro_31!()