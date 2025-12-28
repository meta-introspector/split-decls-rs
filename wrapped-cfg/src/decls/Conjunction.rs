macro_rules! deps {
    () => {
        Literal!();
    };
}

macro_rules! Conjunction {
    () => {
        deps!();
        struct Conjunction { literals : Vec < Literal > , }
    };
}

Conjunction!();