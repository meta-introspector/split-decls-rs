macro_rules! deps {
    () => {
        Assoc!();
    };
}

macro_rules! Affix {
    () => {
        deps!();
        enum Affix { Prefix , Postfix , Infix (Assoc) , }
    };
}

Affix!()