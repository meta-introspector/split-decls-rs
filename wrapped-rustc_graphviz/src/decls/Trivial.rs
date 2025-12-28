macro_rules! deps {
    () => {
        NodeLabels!();
    };
}

macro_rules! Trivial {
    () => {
        deps!();
        type Trivial = NodeLabels < & 'static str > ;
    };
}

Trivial!();