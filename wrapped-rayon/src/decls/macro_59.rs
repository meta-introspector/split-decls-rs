macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! macro_59 {
    () => {
        deps!();
        delegate_iterator ! { IntoIter < K , V > => (K , V) , impl < K : Send , V : Send > }
    };
}

macro_59!();