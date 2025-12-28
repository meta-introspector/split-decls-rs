macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! macro_91 {
    () => {
        deps!();
        delegate_iterator ! { IterMut <'a , T > => &'a mut T , impl <'a , T : Send > }
    };
}

macro_91!()