macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! macro_27 {
    () => {
        deps!();
        delegate_indexed_iterator ! { IntoIter < T > => T , impl < T : Send > }
    };
}

macro_27!()