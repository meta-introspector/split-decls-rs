macro_rules! deps {
    () => {
        RegionConstraintData!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl < 'tcx > RegionConstraintData < 'tcx > { # [doc = " Returns `true` if this region constraint data contains no constraints, and `false`"] # [doc = " otherwise."] pub fn is_empty (& self) -> bool { let RegionConstraintData { constraints , verifys } = self ; constraints . is_empty () && verifys . is_empty () } }
    };
}

impl_146!()