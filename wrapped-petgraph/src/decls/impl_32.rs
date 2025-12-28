macro_rules! deps {
    () => {
        Control!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < B > Control < B > { pub fn breaking () -> Control < () > { Control :: Break (()) } # [doc = " Get the value in `Control::Break(_)`, if present."] pub fn break_value (self) -> Option < B > { match self { Control :: Continue | Control :: Prune => None , Control :: Break (b) => Some (b) , } } }
    };
}

impl_32!()