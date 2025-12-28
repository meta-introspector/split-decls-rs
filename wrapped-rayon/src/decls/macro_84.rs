macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! macro_84 {
    () => {
        deps!();
        delegate_iterator ! { IntoIter < T > => T , impl < T : Send > }
    };
}

macro_84!();