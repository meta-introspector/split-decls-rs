macro_rules! deps {
    () => {
        LocalizedOutlivesConstraintSet!();
        LocalizedOutlivesConstraint!();
    };
}

macro_rules! impl_216 {
    () => {
        deps!();
        impl LocalizedOutlivesConstraintSet { pub (crate) fn push (& mut self , constraint : LocalizedOutlivesConstraint) { if constraint . source == constraint . target && constraint . from == constraint . to { return ; } self . outlives . push (constraint) ; } }
    };
}

impl_216!()