macro_rules! deps {
    () => {
        OutlivesConstraint!();
        OutlivesConstraintSet!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < 'tcx > Index < OutlivesConstraintIndex > for OutlivesConstraintSet < 'tcx > { type Output = OutlivesConstraint < 'tcx > ; fn index (& self , i : OutlivesConstraintIndex) -> & Self :: Output { & self . outlives [i] } }
    };
}

impl_39!();