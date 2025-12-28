macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! macro_1125 {
    () => {
        deps!();
        delegate_indexed_iterator ! { IterMut <'a , T > => &'a mut T , impl <'a , T : Send > }
    };
}

macro_1125!();