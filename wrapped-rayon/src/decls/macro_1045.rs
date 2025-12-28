macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! macro_1045 {
    () => {
        deps!();
        delegate_indexed_iterator ! { Iter <'a , T > => &'a T , impl <'a , T : Sync > }
    };
}

macro_1045!()