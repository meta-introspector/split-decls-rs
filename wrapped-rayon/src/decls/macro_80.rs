macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! macro_80 {
    () => {
        deps!();
        delegate_iterator ! { Drain <'_ , T > => T , impl < T : Send > }
    };
}

macro_80!()