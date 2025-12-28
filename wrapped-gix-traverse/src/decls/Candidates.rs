macro_rules! deps {
    () => {
        Info!();
    };
}

macro_rules! Candidates {
    () => {
        deps!();
        type Candidates = VecDeque < crate :: commit :: Info > ;
    };
}

Candidates!();