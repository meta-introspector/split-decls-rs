macro_rules! deps {
    () => {
        CfgAtom!();
    };
}

macro_rules! Literal {
    () => {
        deps!();
        struct Literal { negate : bool , var : Option < CfgAtom > , }
    };
}

Literal!()