macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! macro_77 {
    () => {
        deps!();
        delegate_iterator ! { Iter <'a , T > => &'a T , impl <'a , T : Sync > }
    };
}

macro_77!()