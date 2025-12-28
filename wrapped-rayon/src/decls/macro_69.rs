macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! macro_69 {
    () => {
        deps!();
        delegate_iterator ! { Drain <'_ , K , V > => (K , V) , impl < K : Send , V : Send > }
    };
}

macro_69!()