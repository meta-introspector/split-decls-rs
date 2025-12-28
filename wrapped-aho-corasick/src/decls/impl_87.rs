macro_rules! deps {
    () => {
        Transition!();
        StateID!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl Transition { # [doc = " Return the byte for which this transition is defined."] pub (crate) fn byte (& self) -> u8 { self . byte } # [doc = " Return the ID of the state that this transition points to."] pub (crate) fn next (& self) -> StateID { self . next } # [doc = " Return the ID of the next transition."] fn link (& self) -> StateID { self . link } }
    };
}

impl_87!();